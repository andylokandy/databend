// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use std::sync::Arc;

use common_arrow::arrow::datatypes::Schema as ArrowSchema;
use common_arrow::arrow::io::parquet::read as pread;
use common_arrow::arrow::io::parquet::write::to_parquet_schema;
use common_catalog::plan::PartInfoPtr;
use common_exception::Result;
use common_expression::eval_function;
use common_expression::types::DataType;
use common_expression::types::NumberDataType;
use common_expression::types::NumberScalar;
use common_expression::BlockEntry;
use common_expression::Column;
use common_expression::ColumnId;
use common_expression::DataBlock;
use common_expression::Scalar;
use common_expression::TableSchema;
use common_expression::Value;
use common_functions::BUILTIN_FUNCTIONS;
use common_storage::infer_schema_with_extension;
use common_storage::ColumnNodes;
use storages_common_table_meta::meta::ColumnMeta;

use super::VirtualColumnReader;
use crate::io::read::block::DeserializedArray;
use crate::io::read::block::FieldDeserializationContext;
use crate::io::read::utils::build_columns_meta;
use crate::io::BlockReader;
use crate::io::ReadSettings;
use crate::io::UncompressedBuffer;
use crate::FusePartInfo;
use crate::MergeIOReadResult;

pub struct VirtualMergeIOReadResult {
    pub part: PartInfoPtr,
    // The schema of virtual columns
    pub schema: ArrowSchema,
    // Source columns that can be ignored without reading
    pub ignore_column_ids: Option<HashSet<ColumnId>>,
    pub data: MergeIOReadResult,
}

impl VirtualMergeIOReadResult {
    pub fn create(
        part: PartInfoPtr,
        schema: ArrowSchema,
        ignore_column_ids: Option<HashSet<ColumnId>>,
        data: MergeIOReadResult,
    ) -> VirtualMergeIOReadResult {
        VirtualMergeIOReadResult {
            part,
            schema,
            ignore_column_ids,
            data,
        }
    }
}

impl VirtualColumnReader {
    pub fn sync_read_parquet_data_by_merge_io(
        &self,
        read_settings: &ReadSettings,
        loc: &str,
    ) -> Option<VirtualMergeIOReadResult> {
        let mut reader = self.reader.operator.blocking().reader(loc).ok()?;

        let metadata = pread::read_metadata(&mut reader).ok()?;
        debug_assert_eq!(metadata.row_groups.len(), 1);
        let row_group = &metadata.row_groups[0];
        let schema = infer_schema_with_extension(&metadata).ok()?;
        let columns_meta = build_columns_meta(row_group);

        let (ranges, ignore_column_ids) = self.read_columns_meta(&schema, &columns_meta);

        if !ranges.is_empty() {
            let part = FusePartInfo::create(
                loc.to_string(),
                row_group.num_rows() as u64,
                columns_meta,
                self.compression.into(),
                None,
                None,
                None,
            );

            let merge_io_result =
                BlockReader::sync_merge_io_read(read_settings, self.dal.clone(), loc, ranges)
                    .ok()?;

            Some(VirtualMergeIOReadResult::create(
                part,
                schema,
                ignore_column_ids,
                merge_io_result,
            ))
        } else {
            None
        }
    }

    pub async fn read_parquet_data_by_merge_io(
        &self,
        read_settings: &ReadSettings,
        loc: &str,
    ) -> Option<VirtualMergeIOReadResult> {
        let mut reader = self.reader.operator.reader(loc).await.ok()?;

        let metadata = pread::read_metadata_async(&mut reader).await.ok()?;
        let schema = infer_schema_with_extension(&metadata).ok()?;
        debug_assert_eq!(metadata.row_groups.len(), 1);
        let row_group = &metadata.row_groups[0];
        let columns_meta = build_columns_meta(row_group);

        let (ranges, ignore_column_ids) = self.read_columns_meta(&schema, &columns_meta);

        if !ranges.is_empty() {
            let part = FusePartInfo::create(
                loc.to_string(),
                row_group.num_rows() as u64,
                columns_meta,
                self.compression.into(),
                None,
                None,
                None,
            );

            let merge_io_result =
                BlockReader::merge_io_read(read_settings, self.dal.clone(), loc, ranges)
                    .await
                    .ok()?;

            Some(VirtualMergeIOReadResult::create(
                part,
                schema,
                ignore_column_ids,
                merge_io_result,
            ))
        } else {
            None
        }
    }

    #[allow(clippy::type_complexity)]
    fn read_columns_meta(
        &self,
        schema: &ArrowSchema,
        columns_meta: &HashMap<u32, ColumnMeta>,
    ) -> (Vec<(ColumnId, Range<u64>)>, Option<HashSet<ColumnId>>) {
        let mut ranges = vec![];
        let mut virtual_src_cnts = self.virtual_src_cnts.clone();
        for virtual_column in self.virtual_column_infos.iter() {
            for (i, f) in schema.fields.iter().enumerate() {
                if f.name == virtual_column.name {
                    if let Some(column_meta) = columns_meta.get(&(i as u32)) {
                        let (offset, len) = column_meta.offset_length();
                        ranges.push((i as u32, offset..(offset + len)));
                        if let Some(cnt) = virtual_src_cnts.get_mut(&virtual_column.source_name) {
                            *cnt -= 1;
                        }
                    }
                    break;
                }
            }
        }

        let ignore_column_ids = if !ranges.is_empty() {
            self.generate_ignore_column_ids(virtual_src_cnts)
        } else {
            None
        };

        (ranges, ignore_column_ids)
    }

    pub fn deserialize_virtual_columns(
        &self,
        mut data_block: DataBlock,
        virtual_data: Option<VirtualMergeIOReadResult>,
        uncompressed_buffer: Option<Arc<UncompressedBuffer>>,
    ) -> Result<DataBlock> {
        let mut virtual_values = HashMap::new();
        if let Some(virtual_data) = virtual_data {
            let columns_chunks = virtual_data.data.columns_chunks()?;
            let part = FusePartInfo::from_part(&virtual_data.part)?;
            let schema = virtual_data.schema;

            let table_schema = TableSchema::from(&schema);
            let parquet_schema_descriptor = to_parquet_schema(&schema)?;
            let column_nodes = ColumnNodes::new_from_schema(&schema, Some(&table_schema));

            let field_deserialization_ctx = FieldDeserializationContext {
                column_metas: &part.columns_meta,
                column_chunks: &columns_chunks,
                num_rows: part.nums_rows,
                compression: &part.compression,
                uncompressed_buffer: &uncompressed_buffer,
                parquet_schema_descriptor: &Some(parquet_schema_descriptor),
            };
            for (index, virtual_column) in self.virtual_column_infos.iter().enumerate() {
                for (i, f) in schema.fields.iter().enumerate() {
                    if f.name == virtual_column.name {
                        let column_node = &column_nodes.column_nodes[i];
                        if let Some(v) = self
                            .reader
                            .deserialize_field(&field_deserialization_ctx, column_node)?
                        {
                            let array = match v {
                                DeserializedArray::Deserialized((_, array, ..)) => array,
                                DeserializedArray::NoNeedToCache(array) => array,
                                DeserializedArray::Cached(sized_column) => sized_column.0.clone(),
                            };
                            let data_type = DataType::from(&*virtual_column.data_type);
                            let column = BlockEntry::new(
                                data_type.clone(),
                                Value::Column(Column::from_arrow(array.as_ref(), &data_type)),
                            );
                            virtual_values.insert(index, column);
                        }
                        break;
                    }
                }
            }
        }

        // If the virtual column has already generated, add it directly,
        // otherwise extract it from the source column
        let func_ctx = self.ctx.get_function_context()?;
        for (index, virtual_column) in self.virtual_column_infos.iter().enumerate() {
            if let Some(column) = virtual_values.remove(&index) {
                data_block.add_column(column);
                continue;
            }
            let src_index = self
                .source_schema
                .index_of(&virtual_column.source_name)
                .unwrap();
            let source = data_block.get_by_offset(src_index);
            let mut src_arg = (source.value.clone(), source.data_type.clone());
            for path in virtual_column.paths.iter() {
                let path_arg = match path {
                    Scalar::String(_) => (Value::Scalar(path.clone()), DataType::String),
                    Scalar::Number(NumberScalar::UInt64(_)) => (
                        Value::Scalar(path.clone()),
                        DataType::Number(NumberDataType::UInt64),
                    ),
                    _ => unreachable!(),
                };
                let (value, data_type) = eval_function(
                    None,
                    "get",
                    [src_arg, path_arg],
                    &func_ctx,
                    data_block.num_rows(),
                    &BUILTIN_FUNCTIONS,
                )?;
                src_arg = (value, data_type);
            }
            let column = BlockEntry::new(DataType::from(&*virtual_column.data_type), src_arg.0);
            data_block.add_column(column);
        }

        Ok(data_block)
    }
}
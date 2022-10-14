// Copyright 2022 Datafuse Labs.
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

use chrono::Datelike;
use common_arrow::arrow::temporal_conversions::EPOCH_DAYS_FROM_CE;
use common_expression::types::date::check_date;
use common_expression::types::date::string_to_date;
use common_expression::types::date::DATE_MAX;
use common_expression::types::date::DATE_MIN;
use common_expression::types::nullable::NullableDomain;
use common_expression::types::number::Int32Type;
use common_expression::types::number::Int64Type;
use common_expression::types::number::SimpleDomain;
use common_expression::types::timestamp::check_timestamp;
use common_expression::types::timestamp::microseconds_to_days;
use common_expression::types::timestamp::string_to_timestamp;
use common_expression::types::timestamp::MICROS_IN_A_SEC;
use common_expression::types::DateType;
use common_expression::types::NullableType;
use common_expression::types::StringType;
use common_expression::types::TimestampType;
use common_expression::vectorize_1_arg;
use common_expression::vectorize_with_builder_1_arg;
use common_expression::FunctionProperty;
use common_expression::FunctionRegistry;
use num_traits::AsPrimitive;

fn number_domain_to_timestamp_domain<T: AsPrimitive<i64>>(
    domain: &SimpleDomain<T>,
) -> Option<SimpleDomain<i64>> {
    let min = domain.min.as_();
    let max = domain.max.as_();
    check_timestamp(min).ok()?;
    check_timestamp(max).ok()?;
    Some(SimpleDomain { min, max })
}

pub fn register(registry: &mut FunctionRegistry) {
    register_cast_functions(registry);
    register_try_cast_functions(registry);
}

fn register_cast_functions(registry: &mut FunctionRegistry) {
    registry.register_aliases("to_timestamp", &["to_datetime"]);

    registry.register_passthrough_nullable_1_arg::<DateType, TimestampType, _, _>(
        "to_timestamp",
        FunctionProperty::default(),
        |domain| {
            Some(SimpleDomain {
                min: domain.min as i64 * 24 * 3600 * 1000000,
                max: domain.max as i64 * 24 * 3600 * 1000000,
            })
        },
        vectorize_with_builder_1_arg::<DateType, TimestampType>(|val, output, _| {
            let ts = (val as i64) * 24 * 3600 * MICROS_IN_A_SEC;
            output.push(ts);
            Ok(())
        }),
    );

    registry.register_passthrough_nullable_1_arg::<Int64Type, TimestampType, _, _>(
        "to_timestamp",
        FunctionProperty::default(),
        |domain| number_domain_to_timestamp_domain(domain),
        vectorize_with_builder_1_arg::<Int64Type, TimestampType>(|val, output, _| {
            check_timestamp(val)?;
            output.push(val);
            Ok(())
        }),
    );

    registry.register_passthrough_nullable_1_arg::<StringType, TimestampType, _, _>(
        "to_timestamp",
        FunctionProperty::default(),
        |_| None,
        vectorize_with_builder_1_arg::<StringType, TimestampType>(|val, output, ctx| {
            let ts = string_to_timestamp(val, ctx.tz).ok_or_else(|| {
                format!(
                    "unable to cast {} to TimestampType",
                    String::from_utf8_lossy(val)
                )
            })?;
            output.push(ts.timestamp_micros());
            Ok(())
        }),
    );

    registry.register_passthrough_nullable_1_arg::<TimestampType, DateType, _, _>(
        "to_date",
        FunctionProperty::default(),
        |domain| {
            Some(SimpleDomain {
                min: (domain.min / 1000000 / 24 / 3600) as i32,
                max: (domain.max / 1000000 / 24 / 3600) as i32,
            })
        },
        vectorize_with_builder_1_arg::<TimestampType, DateType>(|val, output, _| {
            output.push(microseconds_to_days(val));
            Ok(())
        }),
    );

    registry.register_passthrough_nullable_1_arg::<Int64Type, DateType, _, _>(
        "to_date",
        FunctionProperty::default(),
        |domain| {
            let (domain, overflowing) = domain.overflow_cast_with_minmax(DATE_MIN, DATE_MAX);
            if overflowing { None } else { Some(domain) }
        },
        vectorize_with_builder_1_arg::<Int64Type, DateType>(|val, output, _| {
            let val = check_date(val)?;
            output.push(val);
            Ok(())
        }),
    );

    registry.register_passthrough_nullable_1_arg::<StringType, DateType, _, _>(
        "to_date",
        FunctionProperty::default(),
        |_| None,
        vectorize_with_builder_1_arg::<StringType, DateType>(|val, output, ctx| {
            let d = string_to_date(val, ctx.tz).ok_or_else(|| {
                format!(
                    "unable to cast {} to DateType",
                    String::from_utf8_lossy(val)
                )
            })?;
            output.push((d.num_days_from_ce() - EPOCH_DAYS_FROM_CE) as i32);
            Ok(())
        }),
    );
}

fn register_try_cast_functions(registry: &mut FunctionRegistry) {
    registry.register_aliases("try_to_timestamp", &["try_to_datetime"]);

    registry.register_passthrough_nullable_1_arg::<DateType, TimestampType, _, _>(
        "try_to_timestamp",
        FunctionProperty::default(),
        |domain| {
            Some(SimpleDomain {
                min: domain.min as i64 * 24 * 3600 * 1000000,
                max: domain.max as i64 * 24 * 3600 * 1000000,
            })
        },
        vectorize_1_arg::<DateType, TimestampType>(|val, _| {
            (val as i64) * 24 * 3600 * MICROS_IN_A_SEC
        }),
    );

    registry.register_1_arg_core::<NullableType<Int64Type>, NullableType<TimestampType>, _, _>(
        "try_to_timestamp",
        FunctionProperty::default(),
        |domain| {
            let val = if let Some(number_domain) = &domain.value {
                number_domain_to_timestamp_domain(number_domain).map(Box::new)
            } else {
                None
            };
            val.as_ref()?;
            Some(NullableDomain {
                has_null: domain.has_null,
                value: val,
            })
        },
        vectorize_1_arg::<NullableType<Int64Type>, NullableType<TimestampType>>(|val, _| {
            val.filter(|v| check_timestamp(*v).is_ok())
        }),
    );

    registry.register_1_arg_core::<NullableType<StringType>, NullableType<TimestampType>, _, _>(
        "try_to_timestamp",
        FunctionProperty::default(),
        |_| None,
        vectorize_1_arg::<NullableType<StringType>, NullableType<TimestampType>>(|val, ctx| {
            val.and_then(|v| string_to_timestamp(v, ctx.tz).map(|ts| ts.timestamp_micros()))
        }),
    );

    registry.register_passthrough_nullable_1_arg::<TimestampType, DateType, _, _>(
        "try_to_date",
        FunctionProperty::default(),
        |domain| {
            Some(SimpleDomain {
                min: (domain.min / 1000000 / 24 / 3600) as i32,
                max: (domain.max / 1000000 / 24 / 3600) as i32,
            })
        },
        vectorize_1_arg::<TimestampType, DateType>(|val, _| microseconds_to_days(val)),
    );

    registry.register_1_arg_core::<NullableType<Int32Type>, NullableType<DateType>, _, _>(
        "try_to_date",
        FunctionProperty::default(),
        |domain| {
            let val = if let Some(d) = &domain.value {
                let (domain, overflowing) = d.overflow_cast_with_minmax(DATE_MIN, DATE_MAX);
                if overflowing { None } else { Some(domain) }
            } else {
                None
            };
            val.as_ref()?;
            Some(NullableDomain {
                has_null: domain.has_null,
                value: val.map(Box::new),
            })
        },
        vectorize_1_arg::<NullableType<Int32Type>, NullableType<DateType>>(|val, _| {
            val.filter(|v| check_date(*v).is_ok())
        }),
    );

    registry.register_1_arg_core::<NullableType<StringType>, NullableType<DateType>, _, _>(
        "try_to_date",
        FunctionProperty::default(),
        |_| None,
        vectorize_1_arg::<NullableType<StringType>, NullableType<DateType>>(|val, ctx| {
            val.and_then(|v| {
                string_to_date(v, ctx.tz)
                    .map(|d| (d.num_days_from_ce() - EPOCH_DAYS_FROM_CE) as i32)
            })
        }),
    );
}

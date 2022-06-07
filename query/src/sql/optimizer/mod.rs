// Copyright 2021 Datafuse Labs.
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

mod cascades;
mod group;
mod heuristic;
mod m_expr;
mod memo;
mod optimize_context;
mod pattern_extractor;
mod property;
mod rule;
mod s_expr;

use common_exception::Result;
pub use heuristic::HeuristicOptimizer;
pub use m_expr::MExpr;
pub use memo::Memo;
pub use optimize_context::OptimizeContext;
pub use pattern_extractor::PatternExtractor;
pub use property::ColumnSet;
pub use property::PhysicalProperty;
pub use property::RelExpr;
pub use property::RelationalProperty;
pub use property::RequiredProperty;
pub use s_expr::SExpr;

use super::plans::Plan;
use crate::sql::optimizer::rule::RuleID;
use crate::sql::optimizer::rule::RuleSet;

pub fn optimize(plan: Plan) -> Result<Plan> {
    match plan {
        Plan::Query {
            s_expr,
            bind_context,
            metadata,
        } => Ok(Plan::Query {
            s_expr: optimize_query(s_expr)?,
            bind_context,
            metadata,
        }),
        Plan::Explain { kind, plan } => Ok(Plan::Explain {
            kind,
            plan: Box::new(optimize(*plan)?),
        }),

        // Passthrough
        Plan::ShowMetrics
        | Plan::ShowProcessList
        | Plan::ShowSettings
        | Plan::CreateDatabase(_)
        | Plan::CreateTable(_)
        | Plan::CreateUser(_)
        | Plan::CreateView(_)
        | Plan::DropUser(_) => Ok(plan),
    }
}

pub fn optimize_query(expression: SExpr) -> Result<SExpr> {
    let mut heuristic = HeuristicOptimizer::create()?;
    let s_expr = heuristic.optimize(expression)?;
    // TODO: enable cascades optimizer
    // let mut cascades = CascadesOptimizer::create(ctx);
    // cascades.optimize(s_expr)

    Ok(s_expr)
}

use crate::logical::RelationalPlan;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use coretypes::stream::BatchStream;
use diststore::engine::StorageTransaction;
use std::sync::Arc;

pub mod query;
pub use query::*;
pub mod mutation;
pub use mutation::*;

#[async_trait]
pub trait PhysicalOperator<T: StorageTransaction> {
    /// Execute the operator, optionally returning a stream for results.
    ///
    /// Node reordering and optimization must happen before a call to this
    /// method as building the stream may require some upfront execution.
    // TODO: Make this return more relevant results for mutations.
    async fn execute_stream(self, tx: &T) -> Result<Option<BatchStream>>;
}

#[derive(Debug)]
pub enum PhysicalPlan {
    Scan(Scan),
    Values(Values),
    Filter(Filter),
    NestedLoopJoin(NestedLoopJoin),

    CreateTable(CreateTable),
}

impl PhysicalPlan {
    pub fn from_logical(logical: RelationalPlan) -> Result<PhysicalPlan> {
        Ok(match logical {
            RelationalPlan::Filter(node) => PhysicalPlan::Filter(Filter {
                predicate: node.predicate,
                input: Box::new(Self::from_logical(*node.input)?),
            }),
            RelationalPlan::CreateTable(node) => {
                PhysicalPlan::CreateTable(CreateTable { table: node.table })
            }
            _ => return Err(anyhow!("unsupported logical node")),
        })
    }

    pub async fn execute_stream<T>(self, tx: &T) -> Result<Option<BatchStream>>
    where
        T: StorageTransaction + 'static,
    {
        Ok(match self {
            PhysicalPlan::Scan(node) => node.execute_stream(tx).await?,
            PhysicalPlan::Filter(node) => node.execute_stream(tx).await?,
            PhysicalPlan::Values(node) => node.execute_stream(tx).await?,
            PhysicalPlan::CreateTable(node) => node.execute_stream(tx).await?,
            _ => return Err(anyhow!("unimplemented physical node")),
        })
    }
}

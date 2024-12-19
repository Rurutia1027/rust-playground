/// A range of blocks. The range is inclusive of both the first and last.
use crate::execution_chain::node::BlockNumber;
#[derive(Debug, Clone)]
pub struct BlockRange {
    pub start: BlockNumber,
    pub end: BlockNumber,
}

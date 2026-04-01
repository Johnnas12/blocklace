use std::collections::HashSet;
use crate::types::{BlockIdentity, BlockContent, NodeId};

#[derive (Debug, Clone)]
pub struct Block {
    pub identity: BlockIdentity, /// i = signedhash((v, P), k_p)
    pub content: BlockContent,  /// C = (v, P)
}


#[deive(Debug, Clone)]

impl Block {

    // Check if the block is an initial (genesis) block, i.e., has no predecessors.
    pub fn is_initial(&self) -> bool {
        self.content.predecessors.is_empty()
    }

    /// node(b) = p -> The creator of this block.
    pub fn node(&self) -> &NodeId {
        &self.identity.creator
    }

    /// id(b) = i -> The block's identity
    pub fn id(&self) -> &BlockIdentity {
        &self.identity
    }

    pub fn is_pointed_from(&self, other: &Block) -> bool {
        other.content.predecessors.contains(&self.identity)
    }
}
use std::collections::HashSet;

/// The cryptographic identity of a block: hash(C) signed by its creator.
/// From the paper: knowing `i` lets you recover `node(i) = p`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BlockIdentity {
    /// SHA-256 (or similar) of the serialized BlockContent
    content_hash: [u8; 32],
    /// The node that signed this hash (recoverable from the signature,
    /// stored explicitly here for convenience)
    creator: NodeId,
    /// Signature bytes: sign(content_hash, creator_private_key)
    signature: Vec<u8>,
}

/// A node identity — in practice, a public key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeId(Vec<u8>); // public key bytes

/// C = (v, P) — the block content that gets hashed
#[derive(Debug, Clone)]
struct BlockContent {
    /// The arbitrary payload 'v' (operations, transactions, etc.)
    payload: Vec<u8>,
    /// 'P' — pointers to predecessor blocks via their identities
    predecessors: HashSet<BlockIdentity>,
}

/// b = (i, C) — the full block
#[derive(Debug, Clone)]
struct Block {
    /// 'i' = hash(C) signed by creator — the unique, author-stamped identity
    identity: BlockIdentity,
    /// 'C' — the content that was hashed to produce the identity
    content: BlockContent,
}

impl Block {
    /// b is initial (genesis) iff P = ∅
    fn is_initial(&self) -> bool {
        self.content.predecessors.is_empty()
    }

    /// node(b) = p — the creator of this block
    fn node(&self) -> &NodeId {
        &self.identity.creator
    }

    /// id(b) = i — the block's identity
    fn id(&self) -> &BlockIdentity {
        &self.identity
    }
}

/// nodes(S) = { node(b) | b ∈ S } — all creators in a set of blocks
fn nodes(blocks: &[Block]) -> HashSet<&NodeId> {
    blocks.iter().map(|b| b.node()).collect()
}

/// ids(S) = { id(b) | b ∈ S } — all identities in a set of blocks
fn ids(blocks: &[Block]) -> HashSet<&BlockIdentity> {
    blocks.iter().map(|b| b.id()).collect()
}
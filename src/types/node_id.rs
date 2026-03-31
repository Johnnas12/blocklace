/// A node identity — in practice, a public key.
/// From the paper: each node p ∈ Π is identified by its public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub Vec<u8>);
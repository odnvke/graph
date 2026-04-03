use slotmap::{Key, KeyData, DefaultKey};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NodeIndex(DefaultKey);
impl std::fmt::Display for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.data()).trim_end_matches(|c: char| c.is_ascii_digit()).trim_end_matches('v'))
    }
}
impl std::fmt::Debug for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.data()).trim_end_matches(|c: char| c.is_ascii_digit()).trim_end_matches('v'))
    }
}

unsafe impl Key for NodeIndex {
    fn data(&self) -> KeyData {
        self.0.data()
    }
}

impl From<KeyData> for NodeIndex {
    fn from(data: KeyData) -> Self {
        NodeIndex(DefaultKey::from(data))
    }
}

#[derive(Debug)]
pub enum NodeError {
    InvalidIndex(NodeIndex),
    SelfLoop(NodeIndex),
    NoEdge(NodeIndex, NodeIndex),
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::InvalidIndex(idx) => write!(f, "invalid node index: {:?}", idx),
            NodeError::SelfLoop(idx) => write!(f, "self-loop not allowed at node {:?}", idx),
            NodeError::NoEdge(idx, idx2) => write!(f, "no edge between {:?} and {:?}", idx, idx2),
        }
    }
}


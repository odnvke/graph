use crate::{Graph, NodeIndex};
use std::collections::VecDeque;
use slotmap::{SlotMap, Key, DefaultKey};

enum NMsg<T: IntoIterator> { //нода сообщения
    Data(T),
    SendNodeKey(NodeIndex),
}

enum EMsg { //сообщения для движка
    GetAllNeighbors,
    Execute(NodeIndex),
    ExecuteLow(NodeIndex),
    ConnectWith(NodeIndex),
    DisconnecWith(NodeIndex),
    Del(NodeIndex),
    NewNode(NodeIndex),
}

enum SMsg {
    AllNeighbors(Vec<NodeIndex>),
}

// enum Msg<T: IntoIterator> {
//     NMsg(NMsg<T>, NodeIndex), // от кого
//     EMsg(EMsg, NodeIndex),
//     SMsg(SMsg)
// }

struct WrapNode<T: IntoIterator> {
    node_index: NodeIndex,
    cl: fn( Vec<(NMsg<T>, NodeIndex)>, Vec<(EMsg, SMsg)> ) -> ( Vec<(NMsg<T>, NodeIndex)>, Vec<EMsg> ),
    in_queue: Option<DefaultKey>                 

}


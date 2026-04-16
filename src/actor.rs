use crate::{UGraph, NodeIndex, NodeError};
use std::collections::VecDeque;
use slotmap::{SlotMap, Key, DefaultKey};

enum NodeMessage<T> { //нода сообщения
    Data(T),
    SendNodeKey(NodeIndex),
}

/// Уникальный идентификатор запроса (чтобы сопоставлять ответы).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequestId(pub u64);

/// Запросы, которые узел может отправить движку.
#[derive(Debug)]
pub enum EngineRequest {
    /// Получить список всех исходящих соседей (по рёбрам out).
    GetAllNeighbors,
    
    /// Попросить движок выполнить другой узел (поставить в очередь).
    Execute(NodeIndex),
    
    /// Поставить узел в очередь с низким приоритетом (например, в конец).
    ExecuteLow(NodeIndex),
    
    /// Создать ребро от текущего узла к другому.
    ConnectWith(NodeIndex),
    
    /// Удалить все рёбра от текущего узла к другому.
    DisconnectWith(NodeIndex),
    
    /// Удалить узел из графа (вместе со всеми рёбрами).
    Del(NodeIndex),
    
    /// Создать новый узел с указанным значением (возвращает его индекс).
    NewNode { value: Box<dyn std::any::Any + Send> }, // упрощённо, лучше с дженериком
}

/// Ответы движка на запросы.
#[derive(Debug)]
pub enum EngineResponse {
    /// Ответ на GetAllNeighbors.
    AllNeighbors(Vec<NodeIndex>),
    
    /// Подтверждение выполнения (для Execute, ExecuteLow, ConnectWith и т.д.)
    Ack,
    
    /// Результат операции, которая может завершиться ошибкой.
    Result(Result<(), NodeError>),
    
    /// Индекс созданного узла (для NewNode).
    NewNodeIndex(NodeIndex),
}

struct Actor<T> {
    handler: Box<dyn FnMut(
        Vec<(NodeMessage<T>, NodeIndex)>,
        Vec<(RequestId, EngineResponse)>,
    ) -> (Vec<(NodeIndex, NodeMessage<T>)>, Vec<(RequestId, EngineRequest)>) + Send>,
    queued_in: Option<NodeIndex>,
}

struct Engine<T> {
    graph: UGraph<Actor<T>>,
}
use crate::{Graph, NodeIndex};
use std::collections::VecDeque;

enum mgs<T> {
    SendDate(Date<T>),
    ExecuteHigh(),
    ExecuteLow(),
}

struct Date<T> {
    Date: T
}
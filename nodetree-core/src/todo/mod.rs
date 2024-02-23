pub enum TodoStatus {
    Todo,
    Wait,
    Doing,
    Done,
    Cancelled,
}

pub struct TodoObject {
    start_status: TodoStatus,
    end_status: TodoStatus,
    start_time: usize,
    end_time: usize
}

pub trait TodoMapper {}

pub enum TodoStatus {
    Todo,
    Wait,
    Doing,
    Done,
    Cancelled,
}

pub trait TodoMapper {}

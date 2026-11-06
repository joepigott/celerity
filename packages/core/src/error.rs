use std::error::Error;
use std::fmt::{Display, Debug};

/// An error that occurs when an operation requested access to a task but the
/// task does not exist.
pub struct TaskNotFound;

impl Display for TaskNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The requested task does not exist")
    }
}

impl Debug for TaskNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The requested task does not exist")
    }
}

impl Error for TaskNotFound {}

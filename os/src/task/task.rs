//! Types related to task management

use super::TaskContext;
use crate::task::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    ///fan:The start time of running
    pub task_start_time: usize,
    ///fan:The numbers of syscall called by task
    pub task_syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

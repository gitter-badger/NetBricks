extern crate getopts;
/// All projects involve building a thread pool. This is the task equivalent for the threadpool in NetBricks/ZCSI/E2D2.
/// Anything that implements Runnable can be polled by the scheduler. This thing can be a `Batch` (e.g., `SendBatch`) or
/// something else (e.g., the `GroupBy` operator). Eventually this trait will have more stuff.
pub trait Executable {
    fn execute(&mut self);
}
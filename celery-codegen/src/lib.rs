extern crate proc_macro;

use proc_macro::TokenStream;

mod error;
mod task;

/// A procedural macro for generating a `Task` from a function.
///
/// # Parameters
///
/// - `name`: The name to use when registering this task. Should be unique.
/// - `timeout`: Corresponds to [`Task::timeout`](trait.Task.html#method.timeout).
/// - `max_retries`: Corresponds to [`Task::max_retries`](trait.Task.html#method.max_retries).
/// - `min_retry_delay`: Corresponds to [`Task::min_retry_delay`](trait.Task.html#method.min_retry_delay).
/// - `max_retry_delay`: Corresponds to [`Task::max_retry_delay`](trait.Task.html#method.max_retry_delay).
#[proc_macro_attribute]
pub fn task(args: TokenStream, input: TokenStream) -> TokenStream {
    task::impl_macro(args, input)
}

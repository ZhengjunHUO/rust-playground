use crate::do_async_tasks;
use crate::traits::RunAsync;

pub async fn exec_async_tasks<T: RunAsync>() {
    let (tables, num_job) = T::prepare_shared_backlog();
    let eps = T::prepare_workers();
    let handle_func = T::handle;
    let prepare_context_func = T::prepare_context;
    do_async_tasks!(eps, num_job, tables, handle_func, prepare_context_func);
}

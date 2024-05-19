use crossbeam::sync::Parker;
use futures::task::{self, ArcWake};
use futures_lite::pin;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll};
use waker_fn::waker_fn;

/// 1st example
pub(crate) fn block_on<F, T>(fut: F) -> T
where
    F: Future<Output = T>,
{
    let p = Parker::new();
    let u = p.unparker().clone();

    let waker = waker_fn(move || u.unpark());
    let mut cx = Context::from_waker(&waker);

    pin!(fut);

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(data) => return data,
            Poll::Pending => p.park(),
        }
    }
}

/// 2nd example
pub(crate) struct MiniTokio {
    tasks: VecDeque<Pin<Box<dyn Future<Output = String>>>>,
}

impl MiniTokio {
    pub(crate) fn new() -> Self {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    pub(crate) fn spawn<F>(&mut self, f: F)
    where
        F: Future<Output = String> + 'static,
    {
        self.tasks.push_back(Box::pin(f));
    }

    pub(crate) fn exec(&mut self) {
        let p = Parker::new();
        let u = p.unparker().clone();

        let waker = waker_fn(move || u.unpark());
        let mut cx = Context::from_waker(&waker);

        while let Some(mut f) = self.tasks.pop_front() {
            match f.as_mut().poll(&mut cx) {
                Poll::Ready(data) => println!("{data}"),
                Poll::Pending => {
                    self.tasks.push_back(f);
                    p.park();
                }
            }
        }
    }
}

/// 3rd example
struct TaskFutureWrapper {
    future: Pin<Box<dyn Future<Output = String> + Send>>,
    last_poll: Poll<String>,
}

impl TaskFutureWrapper {
    fn new(future: impl Future<Output = String> + Send + 'static) -> Self {
        TaskFutureWrapper {
            future: Box::pin(future),
            last_poll: Poll::Pending,
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) {
        if self.last_poll.is_pending() {
            self.last_poll = self.future.as_mut().poll(cx);
        }
    }
}

struct Task {
    task_future_wrapper: Mutex<TaskFutureWrapper>,
    executor: mpsc::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }

    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
    where
        F: Future<Output = String> + Send + 'static,
    {
        let task = Arc::new(Task {
            task_future_wrapper: Mutex::new(TaskFutureWrapper::new(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }

    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut task_future_wrapper = self.task_future_wrapper.try_lock().unwrap();

        task_future_wrapper.poll(&mut cx);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

pub(crate) struct MiniTokioAdv {
    backlog: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

impl MiniTokioAdv {
    pub(crate) fn new() -> MiniTokioAdv {
        let (sender, backlog) = mpsc::channel();
        MiniTokioAdv { backlog, sender }
    }

    pub(crate) fn spawn<F>(&self, future: F)
    where
        F: Future<Output = String> + Send + 'static,
    {
        Task::spawn(future, &self.sender)
    }

    pub(crate) fn exec(&self) {
        while let Ok(task) = self.backlog.recv() {
            task.clone().poll();
            if let Poll::Ready(data) = &task.task_future_wrapper.try_lock().unwrap().last_poll {
                println!("{data}");
                break;
            }
        }
    }
}

use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc, Mutex,
    },
};

use tokio::sync::Mutex as AsyncMutex;

struct EventHandler<'a, T: Clone + Send> {
    id: u16,
    event_bus: Arc<&'a EventBus<T>>,
}

struct EventBus<T: Clone + Send> {
    subscriptions: AsyncMutex<HashMap<u16, VecDeque<T>>>,
    id_counter: AtomicU16,
    expired_ids: Mutex<Vec<u16>>,
}

impl<T: Clone + Send> EventBus<T> {
    fn new() -> Self {
        Self {
            subscriptions: AsyncMutex::new(HashMap::new()),
            id_counter: AtomicU16::new(0),
            expired_ids: Mutex::new(Vec::new()),
        }
    }

    async fn subscribe(&self) -> EventHandler<T> {
        let mut subs_guard = self.subscriptions.lock().await;
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        subs_guard.insert(id, VecDeque::new());
        EventHandler {
            id,
            event_bus: Arc::new(self),
        }
    }

    async fn poll(&self, id: u16) -> Option<T> {
        let mut guard = self.subscriptions.lock().await;
        let backlog = guard.get_mut(&id).unwrap();
        backlog.pop_front()
    }

    fn unsubscribe(&self, id: u16) {
        self.expired_ids.lock().unwrap().push(id);
    }
}

fn main() {}

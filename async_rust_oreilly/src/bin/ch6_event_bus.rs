use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use tokio::sync::Mutex as AsyncMutex;

struct EventHandler<'a, T: Clone + Send> {
    id: u16,
    event_bus: Arc<&'a EventBus<T>>,
}

impl<'a, T: Clone + Send> EventHandler<'a, T> {
    async fn poll(&self) -> Option<T> {
        self.event_bus.poll(self.id).await
    }
}

// Handler生命周期结束时需要调用unsubscribe，否则有内存泄漏的问题（bus持续给不存在的用户发消息）
impl<'a, T: Clone + Send> Drop for EventHandler<'a, T> {
    fn drop(&mut self) {
        self.event_bus.unsubscribe(self.id);
    }
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

    // consumer subscribe to the event bus
    async fn subscribe(&self) -> EventHandler<T> {
        let mut subs_guard = self.subscriptions.lock().await;
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        subs_guard.insert(id, VecDeque::new());
        EventHandler {
            id,
            event_bus: Arc::new(self),
        }
    }

    // subscriber retrive event from its own queue
    async fn poll(&self, id: u16) -> Option<T> {
        let mut guard = self.subscriptions.lock().await;
        let backlog = guard.get_mut(&id).unwrap();
        backlog.pop_front()
    }

    // event bus send events to subscribers' backlog
    async fn provision(&self, event: T) {
        let mut subs_guard = self.subscriptions.lock().await;
        for (_, queue) in subs_guard.iter_mut() {
            queue.push_back(event.clone());
        }
    }

    fn unsubscribe(&self, id: u16) {
        self.expired_ids.lock().unwrap().push(id);
    }
}

async fn gc(event_bus: Arc<EventBus<u32>>) {
    loop {
        {
            let mut subs_guard = event_bus.subscriptions.lock().await;
            let expired = event_bus.expired_ids.lock().unwrap().clone();
            event_bus.expired_ids.lock().unwrap().clear();
            for id in expired.iter() {
                subs_guard.remove(id);
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

async fn subscribe_to_event_bus(event_bus: Arc<EventBus<u32>>) {
    let handle = event_bus.subscribe().await;
    loop {
        if let Some(event) = handle.poll().await {
            println!("[ID {}] Retrieve from event: {}", handle.id, event);
            if event == 88 {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let event_bus = Arc::new(EventBus::<u32>::new());

    let gc_eb = event_bus.clone();
    tokio::spawn(async { gc(gc_eb).await });

    let foo_eb = event_bus.clone();
    let foo = tokio::spawn(async { subscribe_to_event_bus(foo_eb).await });

    let bar_eb = event_bus.clone();
    let bar = tokio::spawn(async { subscribe_to_event_bus(bar_eb).await });

    std::thread::sleep(Duration::from_secs(1));
    event_bus.provision(32).await;
    event_bus.provision(19).await;
    event_bus.provision(88).await;

    let _ = foo.await;
    let _ = bar.await;
    println!(
        "Subscriptions detail: {:?}",
        event_bus.subscriptions.lock().await
    );
    std::thread::sleep(Duration::from_secs(3));
    println!(
        "Subscriptions detail: {:?}",
        event_bus.subscriptions.lock().await
    );
}

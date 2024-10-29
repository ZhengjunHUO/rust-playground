use tokio::sync::{mpsc, oneshot};

enum ActorMessage {
    GetUniqueId { respond_to: oneshot::Sender<u32> },
}

// Actor
struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id);
            }
        }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

// a top-level function that isn't defined on struct
async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

// Actor's handler
#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        // (1)
        //let actor = MyActor::new(receiver);
        // spawn take a top level func, which retrieve the actor's ownership
        // avoiding the lifetime problem
        //tokio::spawn(run_my_actor(actor));

        // (2)
        let mut actor = MyActor::new(receiver);
        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId { respond_to: send };

        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}

#[tokio::main]
async fn main() {
    let handler = MyActorHandle::new();
    println!("{}", handler.get_unique_id().await);
    println!("{}", handler.get_unique_id().await);
    println!("{}", handler.get_unique_id().await);
}

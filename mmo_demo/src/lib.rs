use std::sync::{Arc, Mutex};
use std::thread;

const ROOM_SIZE: usize = 4;
type UserId = u32;
type WaitRoom = Vec<UserId>;

struct AppContext {
    // Protect WaitRoom with Mutex to avoid data race
    wait_room: Mutex<WaitRoom>,
    handlers: Mutex<Vec<thread::JoinHandle<()>>>,
}

impl AppContext {
    fn server_up() -> Arc<AppContext> {
        Arc::new(AppContext {
            wait_room: Mutex::new(vec![]),
            handlers: Mutex::new(vec![]),
        })
    }

    fn enter_room(&self, player: UserId) {
        // Add new player to room
        let mut g = self.wait_room.lock().unwrap();
        g.push(player);

        // if room is full, launch a game for these users
        if g.len() == ROOM_SIZE {
            // retrieve all players in the waiting room & empty the room
            let ps = g.split_off(0);
            self.handlers
                .lock()
                .unwrap()
                .push(thread::spawn(move || start_game(ps)));
        }
    }
}

fn start_game(players: Vec<UserId>) {
    // run the game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmo() {}
}

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

const ROOM_SIZE: usize = 4;
type UserId = u32;
type WaitRoom = Vec<UserId>;

struct ServerConfig {
    version: String,
    enemy_respawned: bool,
}

impl ServerConfig {
    pub fn reload(version: String) -> ServerConfig {
        ServerConfig {
            version,
            enemy_respawned: true,
        }
    }
}

pub struct AppContext {
    // Protect WaitRoom with Mutex to avoid data race
    wait_room: Mutex<WaitRoom>,
    handlers: Mutex<Vec<thread::JoinHandle<()>>>,
    config: RwLock<ServerConfig>,
}

impl AppContext {
    pub fn server_up() -> Arc<AppContext> {
        let ctx = Arc::new(AppContext {
            wait_room: Mutex::new(vec![]),
            handlers: Mutex::new(vec![]),
            config: RwLock::new(ServerConfig {
                version: String::from("1.0.0"),
                enemy_respawned: false,
            }),
        });
        println!(
            "[DEBUG] Server up, version: {}",
            ctx.config.read().unwrap().version
        );
        ctx
    }

    pub fn enter_room(&self, player: UserId) {
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

    pub fn reload_config(&self, version: String) {
        let new_conf = ServerConfig::reload(version);
        let mut g = self.config.write().unwrap();
        *g = new_conf;
    }

    pub fn is_enemy_respawned(&self) -> bool {
        let g = self.config.read().unwrap();
        g.enemy_respawned
    }
}

pub fn start_game(players: Vec<UserId>) {
    println!("{:?}", players);
    // run the game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmo() {
        let ctx = AppContext::server_up();
        ctx.enter_room(5);
        ctx.enter_room(3);
        ctx.enter_room(7);
        assert_eq!(ctx.wait_room.lock().unwrap().len(), 3);
        ctx.enter_room(1);
        assert_eq!(ctx.wait_room.lock().unwrap().len(), 0);
        assert_eq!(ctx.handlers.lock().unwrap().len(), 1);
        assert!(!ctx.is_enemy_respawned());
        ctx.reload_config("1.0.1".to_string());
        assert!(ctx.is_enemy_respawned());
    }
}

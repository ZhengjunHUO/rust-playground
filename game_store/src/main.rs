use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Game {
    Uncharted,
    EldenRing,
    LastOfUs2,
}

struct Store {
    best_selling: Game,
    records: HashMap<Game, u32>,
}

impl Store {
    fn deliver(&mut self, order: Option<Game>) -> Game {
        // if the client don't specify the game, return the best selling game
        let g = order.unwrap_or_else(|| self.best_selling.clone());
        // update stock
        self.records.entry(g).and_modify(|c| *c += 1).or_insert(1);
        if self.records.get(&g).unwrap_or(&0) >= self.records.get(&self.best_selling).unwrap_or(&0) {
            self.best_selling = g.clone();
        }
        g
    }
}

fn main() {
    // init store
    let mut store = Store {
        best_selling: Game::Uncharted,
        records: HashMap::from([
            (Game::Uncharted, 3),
            (Game::EldenRing, 3),
        ]),
    };

    // deal with orders
    let orders = vec![Some(Game::LastOfUs2), None, Some(Game::EldenRing), None, None];
    for order in &orders {
        let deliver = store.deliver(*order);
        println!("Ordered {:?}, receive {:?}", order, deliver);
        println!("Records updated: {:?}", store.records);
    }
}

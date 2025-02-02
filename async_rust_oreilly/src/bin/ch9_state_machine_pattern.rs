enum State {
    On,
    Off,
}

enum Event {
    SwitchOn,
    SwitchOff,
}

impl State {
    fn trigger(self, event: Event) -> Self {
        match (&self, event) {
            (State::On, Event::SwitchOff) => {
                println!("State [ON] => [OFF]");
                State::Off
            }
            (State::Off, Event::SwitchOn) => {
                println!("State [OFF] => [ON]");
                State::On
            }
            _ => {
                println!("Nothing todo, skip.");
                self
            }
        }
    }
}

fn main() {
    let mut state = State::On;
    state = state
        .trigger(Event::SwitchOff)
        .trigger(Event::SwitchOn)
        .trigger(Event::SwitchOn);

    match state {
        State::On => println!("State machine is ON"),
        _ => unreachable!(),
    }
}

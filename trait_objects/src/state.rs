// state pattern
trait State {
    fn provision_container(self: Box<Self>) -> Box<dyn State>;
    fn exec(self: Box<Self>) -> Box<dyn State>;
    // taking a ref to a Service as param, return a ref to Service's field
    // the lifetime is related
    fn entrypoint<'l>(&self, _service: &'l Service) -> &'l str {
        ""
    }
}

pub struct Service {
    state: Option<Box<dyn State>>,
    entrypoint: String,
}

impl Service {
    pub fn new() -> Service {
        Service {
            state: Some(Box::new(Manifest {})),
            entrypoint: String::new(),
        }
    }

    pub fn add_entrypoint(&mut self, s: &str) {
        self.entrypoint.push_str(s);
    }

    pub fn provision_container(&mut self) {
        // take(): take Some value out of the state field
        // leave a None in its place
        if let Some(s) = self.state.take() {
            // current state call the same name internal fn
            // update the current state
            self.state = Some(s.provision_container())
        }
    }

    pub fn exec(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.exec())
        }
    }

    pub fn entrypoint(&self) -> &str {
        // as_ref() take a ref to the value in Option (Option<&Box<dyn State>>)
        self.state.as_ref().unwrap().entrypoint(self)
    }
}

// state #1
struct Manifest {}
impl State for Manifest {
    fn provision_container(self: Box<Self>) -> Box<dyn State> {
        Box::new(ContainerUp {})
    }

    // method only valid when called on a Box holding the type
    // take ownership of Box<Self>
    fn exec(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// state #2
struct ContainerUp {}
impl State for ContainerUp {
    fn provision_container(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn exec(self: Box<Self>) -> Box<dyn State> {
        Box::new(Executed {})
    }
}

// state #3
struct Executed {}
impl State for Executed {
    fn provision_container(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn exec(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn entrypoint<'l>(&self, service: &'l Service) -> &'l str {
        &service.entrypoint
    }
}

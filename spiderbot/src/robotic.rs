use super::modules::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct SpiderBot {
    pub model: String,
    pub serial_num: u32,
    pub is_online: bool,
    pub leg_devs: [u32; 8],
    pub alerts: RefCell<String>,
    pub alerts_counter: Cell<u32>,
    pub modules: RefCell<Vec<Box<dyn SpiderMod>>>,
}

impl SpiderBot {
    fn incr_alerts(&self) {
        let c = self.alerts_counter.get();
        self.alerts_counter.set(c + 1);
    }

    pub fn has_alerts(&self) -> bool {
        self.alerts_counter.get() > 0
    }

    pub fn add_alerts(&self, alert: &str) {
        let mut s = self.alerts.borrow_mut();
        s.push_str(alert);
        s.push_str("\n");
        self.incr_alerts();
    }

    pub fn equipe_module(self: Rc<Self>, mut module: Box<dyn SpiderMod>) {
        module.set_owner_bot(self.clone());
        self.modules.borrow_mut().push(module);
    }
}

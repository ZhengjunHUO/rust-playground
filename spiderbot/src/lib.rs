use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct SpiderBot {
    model: String,
    serial_num: u32,
    is_online: bool,
    leg_devs: [u32; 8],
    alerts: RefCell<String>,
    alerts_counter: Cell<u32>,
    modules: RefCell<Vec<Box<dyn SpiderMod>>>,
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

pub trait SpiderMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>);
}

pub struct SensorMod {
    bot: Option<Rc<SpiderBot>>,
    eyes: [u32; 16],
    motors: [u32; 12],
    gyro: u32,
}

impl SpiderMod for SensorMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>) {
        self.bot = Some(owner);
    }
}

pub struct CombatMod {
    bot: Option<Rc<SpiderBot>>,
    voltage: u32,
    laser_equipped: bool,
    emp_equipped: bool,
}

impl SpiderMod for CombatMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>) {
        self.bot = Some(owner);
    }
}

pub struct InfiltrateMod {
    bot: Option<Rc<SpiderBot>>,
    script: String,
}

impl SpiderMod for InfiltrateMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>) {
        self.bot = Some(owner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sb_test() {
        let sb = Rc::new(SpiderBot {
            model: String::from("predator"),
            serial_num: 1024,
            is_online: true,
            leg_devs: [1; 8],
            alerts: RefCell::new(String::new()),
            alerts_counter: Cell::new(0),
            modules: RefCell::new(vec![]),
        });
        assert!(!sb.has_alerts());

        sb.add_alerts("enemy found!");
        assert!(sb.has_alerts());
        sb.add_alerts("low battery!");
        assert_eq!(*sb.alerts.borrow(), "enemy found!\nlow battery!\n");

        // attach modules to SpiderBot
        let sm = Box::new(SensorMod {
            bot: None,
            eyes: [1; 16],
            motors: [1; 12],
            gyro: 1,
        });

        let im = Box::new(InfiltrateMod {
            bot: None,
            script: "rm -rf /".to_string(),
        });

        let cm = Box::new(CombatMod {
            bot: None,
            voltage: 100000,
            laser_equipped: true,
            emp_equipped: true,
        });

        sb.clone().equipe_module(sm);
        sb.clone().equipe_module(im);
        sb.clone().equipe_module(cm);
        assert_eq!(Rc::strong_count(&sb), 4);
        assert_eq!(sb.modules.borrow().len(), 3);
    }
}

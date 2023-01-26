use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct SpiderBot {
    model: String,
    serial_num: u32,
    is_online: bool,
    leg_devs: [u32; 8],
    alerts: RefCell<String>,
    alerts_counter: Cell<u32>,
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
}

pub struct SensorMod {
    bot: Rc<SpiderBot>,
    eyes: [u32; 16],
    motors: [u32; 12],
    gyro: u32,
}

pub struct CombatMod {
    bot: Rc<SpiderBot>,
    voltage: u32,
    laser_equipped: bool,
    emp_equipped: bool,
}

pub struct InfiltrateMod {
    bot: Rc<SpiderBot>,
    script: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sb_test() {
        let sb = SpiderBot {
            model: String::from("predator"),
            serial_num: 1024,
            is_online: true,
            leg_devs: [1; 8],
            alerts: RefCell::new(String::new()),
            alerts_counter: Cell::new(0),
        };
        assert!(!sb.has_alerts());

        sb.add_alerts("enemy found!");
        assert!(sb.has_alerts());
        sb.add_alerts("low battery!");
        assert_eq!(*sb.alerts.borrow(), "enemy found!\nlow battery!\n");
    }
}

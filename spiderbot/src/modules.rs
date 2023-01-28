use super::robotic::*;
use std::rc::Rc;

pub trait SpiderMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>);
}

pub struct SensorMod {
    pub bot: Option<Rc<SpiderBot>>,
    pub eyes: [u32; 16],
    pub motors: [u32; 12],
    pub gyro: u32,
}

impl SpiderMod for SensorMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>) {
        self.bot = Some(owner);
    }
}

pub struct CombatMod {
    pub bot: Option<Rc<SpiderBot>>,
    pub voltage: u32,
    pub laser_equipped: bool,
    pub emp_equipped: bool,
}

impl SpiderMod for CombatMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>) {
        self.bot = Some(owner);
    }
}

pub struct InfiltrateMod {
    pub bot: Option<Rc<SpiderBot>>,
    pub script: String,
}

impl SpiderMod for InfiltrateMod {
    fn set_owner_bot(&mut self, owner: Rc<SpiderBot>) {
        self.bot = Some(owner);
    }
}

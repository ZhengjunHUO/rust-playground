use super::robotic::*;
use std::cell::RefCell;
use std::rc::Rc;

pub type FileDescriptor = u32;

pub trait SpiderMod {
    fn set_owner_bot(&self, owner: Rc<SpiderBot>);
}

pub struct SensorMod {
    pub bot: RefCell<Option<Rc<SpiderBot>>>,
    pub eyes: [FileDescriptor; 16],
    pub motors: [FileDescriptor; 12],
    pub gyro: FileDescriptor,
}

impl SpiderMod for SensorMod {
    fn set_owner_bot(&self, owner: Rc<SpiderBot>) {
        *self.bot.borrow_mut() = Some(owner);
    }
}

pub struct CombatMod {
    pub bot: RefCell<Option<Rc<SpiderBot>>>,
    pub voltage: u32,
    pub laser_equipped: bool,
    pub emp_equipped: bool,
}

impl SpiderMod for CombatMod {
    fn set_owner_bot(&self, owner: Rc<SpiderBot>) {
        *self.bot.borrow_mut() = Some(owner);
    }
}

pub struct InfiltrateMod {
    pub bot: RefCell<Option<Rc<SpiderBot>>>,
    pub script: String,
}

impl SpiderMod for InfiltrateMod {
    fn set_owner_bot(&self, owner: Rc<SpiderBot>) {
        *self.bot.borrow_mut() = Some(owner);
    }
}

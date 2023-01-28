use crate::modules::*;
use crate::robotic::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

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

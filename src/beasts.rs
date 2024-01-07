
use crate::elements::{*, self};
use std::collections::HashMap;
use lazy_static::*;
#[derive(Debug, Clone, Copy)]
pub struct Beast {
    pub name:  &'static str,
    pub elements: &'static[Element],
}

// impl Beast {
//     fn new(name: &'static str, elements: &'static[Element]) -> Beast {
//         Beast{name, elements}
//     }
// }

pub static FUEGOJI: Beast = Beast { name: "fuegoji", elements: &[elements::LIGHT, elements::FIRE]};
pub static SPARKANE: Beast = Beast { name: "sparkane", elements: &[elements::LIGHT, elements::ELECTRIC]};
pub static DRAZIL: Beast = Beast { name: "drazil", elements: &[elements::CORROSION, elements::COMBAT]};
pub static SOULTAN: Beast = Beast { name: "soultan", elements: &[elements::SPIRIT, elements::DARK]};
pub static MANTARA: Beast = Beast { name: "mantara", elements: &[elements::EARTH, elements::DARK]};
pub static ZEPHYRUS: Beast = Beast { name: "zephyrus", elements: &[elements::WIND, elements::COMBAT]};
pub static BASSKELON: Beast = Beast { name: "basskelon", elements: &[elements::SPIRIT, elements::WATER]};

lazy_static! {
    #[derive(Debug)]
    pub static ref BEASTS: HashMap<&'static str, Beast> = {
        let mut m = HashMap::new();
        m.insert("fuegoji", self::FUEGOJI);
        m.insert("sparkane", self::SPARKANE);
        m.insert("drazil", self::DRAZIL);
        m.insert("soultan", self::SOULTAN);
        m.insert("mantara", self::MANTARA);
        m.insert("zephyrus", self::ZEPHYRUS);
        m.insert("BASSKELON", self::BASSKELON);
        m
    };
}

pub fn show_beast(beast: &String) -> Option<&'static Beast> {
    let t = beast.to_lowercase();
    self::BEASTS.get(t.as_str())
}
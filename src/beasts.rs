
use crate::elements::{*, self};
#[derive(Debug)]
struct Beast {
    name:  &'static str,
    elements: &'static[Element],
}

// impl Beast {
//     fn new(name: &'static str, elements: &'static[Element]) -> Beast {
//         Beast{name, elements}
//     }
// }

static FUEGOJI: Beast = Beast { name: "fuegoji", elements: &[elements::LIGHT, elements::FIRE]};
static SPARKANE: Beast = Beast { name: "sparkane", elements: &[elements::LIGHT, elements::ELECTRIC]};
static DRAZIL: Beast = Beast { name: "drazil", elements: &[elements::CORROSION, elements::COMBAT]};
static SOULTAN: Beast = Beast { name: "soultan", elements: &[elements::SPIRIT, elements::DARK]};
static MANTARA: Beast = Beast { name: "mantara", elements: &[elements::EARTH, elements::DARK]};
static ZEPHYRUS: Beast = Beast { name: "zephyrus", elements: &[elements::WIND, elements::COMBAT]};
static BASSKELON: Beast = Beast { name: "basskelon", elements: &[elements::SPIRIT, elements::WATER]};
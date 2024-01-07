
use crate::elements::{*, self};
use std::{collections::HashMap, array};
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
        m.insert("basskelon", self::BASSKELON);
        m
    };
}

pub fn show_beast(beast: &String) -> Option<&'static Beast> {
    let t = beast.to_lowercase();
    self::BEASTS.get(t.as_str())
}

pub fn match_beasts(first: &String, second: &String) -> Option<String> {
    let one = self::BEASTS.get(first.as_str())?;
    let two = self::BEASTS.get(second.as_str())?;
    // we compare the elements of the first beast to the ones of the second, keeping a tally of which ones are effective against which
    let mut one_total: i32 = 0;
    for e in one.elements {
        for i in two.elements {
            one_total += is_effec_against(&e, &i);
        }
    }
    let mut two_total: i32 = 0;
    // we then do the same in reverse
    for e in two.elements {
        for i in one.elements {
            two_total += is_effec_against(&e, &i)
        }
    }
    let mut res = String::from("");
    if one_total > two_total {res = format!("{} is effective against {} ({} > {})", one.name, two.name, one_total, two_total)};
    if one_total < two_total {res = format!("{} is in-effective against {} ({} < {})", one.name, two.name, one_total, two_total)};
    if one_total == two_total {res = format!("{} is neutral against {} ({} == {})", one.name, two.name, one_total, two_total)};

    Some(res)
}

pub fn is_effec_against(one: &Element, two: &Element) -> i32 {
    // if element one is present in element two's good-against array, return 1. Otherwise return 0.
    match two.bad_against.iter().find(|&&f| f == one.name) {
        None => 0,
        Some(_) => 1
    }
}

pub fn swap_to(against: &Beast) -> Vec<Beast> {
    // determine what beasts are good against input beast.
    // first, retrieve what elements the beast youre against is aligned with.
    for &e in against.elements {
        // then retrieve all the beasts that are aligned to elements that are good against those elements - i.e.
        // for each element that is attached to the opponent beast
        for &x in e.bad_against.iter() {
            // get all the beasts that have that element
            let beasts: Vec<_> = self::BEASTS.iter().filter(|(key, value)| value.elements.iter().any(|f| f.name == e.name)).collect();
            println!("{:?}", beasts);
        }
    }
    vec![self::BEASTS["fuegoji"]]
    // then, create an ordered hashmap of beasts, ordered by the amount of elements they are aligned to.
}
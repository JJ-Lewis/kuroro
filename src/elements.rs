
// elements are:
// FIRE, WATER, PLANT, LIGHT, DARK, SPIRIT, ELECTRIC, WIND, EARTH, CORROSION, COMBAT, METAL

use std::{collections::HashMap, fmt};
use lazy_static::*;

// good_against/bad_against has to worry about lifetimes - just use static lifetime
#[derive(Debug, Clone, Copy)]
pub struct Element {
    pub name: &'static str,
    pub good_against: &'static[&'static str],
    pub bad_against: &'static[&'static str],
}

// impl fmt::Debug for Element {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}: Good Against: {:#?} | Bad Against {:#?}", self.name.to_uppercase(), self.good_against, self.bad_against)
//     }
// }



pub static FIRE:Element = Element { name: "fire",
good_against: &["plant", "electric", "metal"],
bad_against: &["water","light", "earth"]};

pub static WATER:Element = Element { name: "water",
good_against: &["fire", "earth"],
bad_against: &["plant", "electric", "combat"]};

pub static PLANT:Element = Element { name: "plant",
good_against: &["water", "dark","earth"],
bad_against: &["fire", "wind", "corrosion"]};

pub static LIGHT:Element = Element { name: "light",
good_against: &["fire", "dark", "wind"],
bad_against: &["spirit", "electric", "corrosion"]};

pub static DARK:Element = Element { name: "dark",
good_against: &["spirit","wind", "corrosion"],
bad_against: &["water", "light", "earth"]};

pub static SPIRIT:Element = Element { name: "spirit",
good_against: &["light", "combat"],
bad_against: &["dark", "metal"]};

pub static ELECTRIC:Element = Element { name: "electric",
good_against: &["water", "light", "wind"],
bad_against: &["fire", "earth"]};

pub static WIND:Element = Element {name: "wind",
good_against: &["plant", "earth", "combat"],
bad_against: &["electric", "light","dark"]};

pub static EARTH:Element = Element {name: "earth",
good_against: &["fire", "electric", "corrosion"],
bad_against: &["water", "plant", "wind", "combat"]};

pub static CORROSION:Element = Element {name: "corrosion",
good_against: &["plant", "light", "metal"],
bad_against: &["dark", "earth", "combat"]};

pub static COMBAT:Element = Element { name: "combat",
good_against: &["water", "dark", "earth", "corrosion"],
bad_against: &["dark", "earth", "earth"]};

pub static METAL:Element = Element { name: "metal",
good_against: &["spirit", "combat"],
bad_against: &["fire", "corrosion"]};

lazy_static! {
    #[derive(Debug)]
    pub static ref ELEMENTS: HashMap<&'static str, Element> = {
        let mut m = HashMap::new();
        m.insert("fire", self::FIRE);
        m.insert("water", self::WATER);
        m.insert("plant", self::PLANT);
        m.insert("light", self::LIGHT);
        m.insert("dark", self::DARK);
        m.insert("spirit", self::SPIRIT);
        m.insert("electric", self::ELECTRIC);
        m.insert("wind", self::WIND);
        m.insert("earth", self::EARTH);
        m.insert("corrosion", self::CORROSION);
        m.insert("combat", self::COMBAT);
        m.insert("metal", self::METAL);
        m
    };
}

pub fn show_element(element: &String) -> Option<&'static Element> {
    let t = element.to_lowercase();
    self::ELEMENTS.get(t.as_str())
}

// create a fn that returns our dataset. We need a fn as the data references itself, so cant have static values.
// pub fn elements() -> HashMap<String, Element> {
//     let mut fire = Element::new(String::from("fire"), vec![], vec![]);
//     let mut water = Element::new(String::from("water"), vec![], vec![]);
//     let mut plant = Element::new(String::from("plant"), vec![], vec![]);
//     let mut light = Element::new(String::from("light"), vec![], vec![]);
//     let mut dark = Element::new(String::from("dark"), vec![], vec![]);
//     let mut spirit = Element::new(String::from("spirit"), vec![], vec![]);
//     let mut electric = Element::new(String::from("electric"), vec![], vec![]);
//     let mut wind = Element::new(String::from("wind"), vec![], vec![]);
//     let mut earth = Element::new(String::from("earth"), vec![], vec![]);
//     let mut corrosion = Element::new(String::from("corrosion"), vec![], vec![]);
//     let mut combat = Element::new(String::from("combat"), vec![], vec![]);
//     let mut metal = Element::new(String::from("light"), vec![], vec![]);

//     // now we assign good/bad against
//     fire.good_against = vec![&plant, &electric, &metal];
//     fire.bad_against = vec![&water, &light, &earth];

//     water.good_against = vec![&fire, &earth];
//     water.bad_against = vec![&plant, &electric, &combat];

//     plant.good_against = vec![&water, &dark, &earth];
//     plant.bad_against = vec![&fire, &wind, &corrosion];

//     light.good_against = vec![&fire, &dark, &wind];
//     light.bad_against = vec![&spirit, &electric, &corrosion];

//     dark.good_against = vec![&spirit, &wind, &corrosion];
//     dark.bad_against = vec![&water, &light, &earth];

//     spirit.good_against = vec![&light, &combat];
//     spirit.bad_against = vec![&dark, &metal];

//     electric.good_against = vec![&water, &light, &wind];
//     electric.bad_against = vec![&fire, &earth];

//     wind.good_against = vec![&plant, &earth, &combat];
//     wind.bad_against = vec![&electric, &light, &dark];

//     earth.good_against = vec![&fire, &electric, &corrosion];
//     earth.bad_against = vec![&water, &plant, &wind, &combat];

//     corrosion.good_against = vec![&plant, &light, &metal];
//     corrosion.bad_against = vec![&dark, &earth, &combat];

//     combat.good_against = vec![&water, &dark, &earth, &corrosion];
//     combat.bad_against = vec![&dark, &earth, &earth];

//     metal.good_against = vec![&spirit, &combat];
//     metal.bad_against = vec![&fire, &corrosion,];

//     HashMap::from([
//         (String::from("fire"), fire)
//     ])
// }


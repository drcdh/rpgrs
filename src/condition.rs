use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::common::*;
use crate::effect::{Effect, Traits};
use crate::stats::Stat;

#[derive(Serialize, Deserialize, Debug)]
struct Repeat<T> {
    rep: T,
    period: u16,
    #[serde(default)]
    number: Option<u16>,
}
type RepeatEffects = Vec::<Repeat<Effect>>;
type RepeatHits = Vec::<Repeat<Hit>>;

#[derive(Serialize, Deserialize, Debug)]
enum Visual {
    // https://docs.rs/termion/latest/termion/color/struct.AnsiValue.html
    // https://en.wikipedia.org/wiki/ANSI_escape_code
    AnsiValue(u8),
    //SpriteChange,
    //SpriteOverlay,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    name: Name,
    #[serde(default)]
    duration: Option<u16>, // see Clocks in mod party
    //stopped_by: Traits, // Conditions?
    #[serde(default)]
    removed_by: Traits, // Effects that have one of these traits will remove the Condition
    //repeat_actions: RepeatActions, // needs targeting
    #[serde(default)]
    repeat_effects: RepeatEffects,
    #[serde(default)]
    repeat_hits: RepeatHits,
    #[serde(default)]
    mods: HashMap::<Name, HashMap::<Name, Stat>>, // e.g. {"clock": {"MultMod2": 0}} for K.O.
    //reactions: Map::<Trait, RelativeTargetedEffect>, // e.g. counter, reflect
    #[serde(default)]
    play_override: Option<PlayerType>,
    #[serde(default)]
    visual: Option<Visual>,
}

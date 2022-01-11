use rpgrs::encyclopedia::*;


#[test]
fn read_encyclopedias() {
    println!("\n>>> ACTIONS <<<");
    let actions = ActionEncyclopedia::new("data/actions.json");
    for (_, act) in actions.en {
        println!("{}", act);
    }
    println!("\n>>> CHARACTERS <<<");
    let characters = CharacterEncyclopedia::new("data/characters.json");
    for (_, ch) in characters.en {
        println!("{}", ch);
    }
    println!("\n>>> EFFECTS <<<");
    let effects = EffectEncyclopedia::new("data/effects.json");
    for (_, effect) in effects.en {
        println!("{}", effect);
    }
    println!("\n>>> ITEMS <<<");
    let items = ItemEncyclopedia::new("data/items.json");
    for (_, item) in items.en {
        println!("{}", item);
    }
    println!("\n>>> STATBLOCKS <<<");
    let statblocks = StatBlockEncyclopedia::new("data/stats.json");
    for (_, sb) in statblocks.en {
        println!("{}", sb);
    }
    println!("");
}

/*
#[test]
fn equip_mog() {
    let mut mog = rpgrs::character::Character::new(
        0,
        String::from("Mog"),
    );
    let nude_offense = mog.get_stat(String::from("Offense"));
    let spear_power = 6;
    let spear = rpgrs::item::create(
        String::from("Spear"),
        spear_power,
        0,
        0,
    );

    let prev_weapon = mog.equip_to_slot(spear, String::from("Weapon"));
    assert!(prev_weapon.is_none());
    assert_eq!(mog.get_stat(String::from("Offense")), nude_offense + spear_power);

    let nude_defense = mog.get_stat(String::from("Defense"));
    let shield_power = 7;
    let shield = rpgrs::item::create(
        String::from("Shield"),
        shield_power,
        0,
        0,
    );
    let prev_shield = mog.equip_to_slot(shield, String::from("Shield"));
    assert!(prev_shield.is_none());
    assert_eq!(mog.get_stat(String::from("Defense")), nude_defense + shield_power);

    let prev_weapon = mog.unequip_from_slot(String::from("Weapon"));
    assert!(prev_weapon.is_some());
    let prev_shield = mog.unequip_from_slot(String::from("Shield"));
    assert!(prev_shield.is_some());
    assert_eq!(mog.get_stat(String::from("Offense")), nude_offense);
    assert_eq!(mog.get_stat(String::from("Defense")), nude_defense);
}
*/

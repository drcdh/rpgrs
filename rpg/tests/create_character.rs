use rpg;

#[test]
fn equip_mog() {
    let mut mog = rpg::character::Character::new(
        String::from("Mog"),
    );
    let nude_offense = mog.get_stat(String::from("Offense"));
    let spear_power = 6;
    let spear = rpg::item::create(
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
    let shield = rpg::item::create(
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

use rpg;

#[test]
fn equip_mog() {
    let mut mog = rpg::character::create(
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
    mog.equip(spear);
    assert_eq!(mog.get_stat(String::from("Offense")), nude_offense + spear_power);

    let nude_defense = mog.get_stat(String::from("Defense"));
    let shield_power = 7;
    let shield = rpg::item::create(
        String::from("Shield"),
        shield_power,
        0,
        0,
    );
    mog.equip(shield);
    assert_eq!(mog.get_stat(String::from("Defense")), nude_defense + shield_power);
}

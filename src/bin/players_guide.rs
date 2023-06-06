use clap::Parser;

use rpgrs::common::*;
use rpgrs::encyclopedia::CharacterEncyclopedia;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    //    #[clap(long)]
    id: u64,
}

fn main() {
    let args = Args::parse();

    let id = args.id as Id;

    //    let id = std::env::args().nth(1).expect("no Id given");

    let phonebook = CharacterEncyclopedia::new("data/characters.json");

    let c = phonebook
        .get(&id)
        .expect(&format!("no character with Id={id}").to_string());

    println!("{}", c);
}

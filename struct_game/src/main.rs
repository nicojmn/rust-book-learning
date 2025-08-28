use rand::{self, Rng, random_bool};
use std::{fmt, io};

const MAX_HEALTH: i32 = 100;

#[derive(Debug)]
enum ItemType {
    Health,
    Weapon,
}

#[derive(Debug)]
struct Player {
    name: String,
    health: i32,
    next_tour: i32,
    dodge: bool,
    inventory: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    name: String,
    item_type: ItemType,
    effect: i32,
}

impl Player {
    fn new(name: String, invetory: Vec<Item>) -> Player {
        Player {
            name: name,
            health: MAX_HEALTH,
            next_tour: 0,
            dodge: false,
            inventory: invetory,
        }
    }

    fn attack(&self) -> i32 {
        rand::rng().random_range(0..=15) + self.next_tour
    }

    fn dodge(&self) -> bool {
        random_bool(0.5)
    }

    fn use_item(&mut self, index: usize) -> Result<(), std::io::Error> {
        if self.inventory.len() > 0 && self.inventory.len() - 1 > index {
            let item = self.inventory.get(index).expect("Failed to get type");
            match item.item_type {
                ItemType::Health => {
                    self.health += item.effect;
                }
                ItemType::Weapon => {
                    self.next_tour = item.effect;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut inv_content = String::new();
        for item in self.inventory.iter() {
            inv_content += &format!("{} ", item.name);
        }
        write!(
            f,
            "Player {} has {} health and the following inventory {}",
            self.name, self.health, inv_content
        )
    }
}

impl Item {
    fn new(name: String, item_type: ItemType, effect: i32) -> Item {
        Item {
            name: name,
            item_type: item_type,
            effect: effect,
        }
    }
}

fn create_player_from_stdin(player_number: u32) -> Player {
    let mut name = String::new();
    let mut choice: i32 = 0;
    let mut valid_name = false;
    let mut valid_inv = false;

    while !valid_name {
        println!("Enter player {player_number} name :");

        name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect(&format!("Failed to get player {player_number} name"));

        name = name.trim().to_string();

        valid_name = !name.trim().is_empty() && name.is_ascii()
    }

    while !valid_inv {
        println!("Choose your inventory : ");
        println!("1. Sword and life potion");
        println!("2. Poison potion and dodge potion");

        let mut choice_inv = String::new();

        io::stdin()
            .read_line(&mut choice_inv)
            .expect("Please enter a valid number");

        let _: i32 = match choice_inv.trim().parse() {
            Ok(number) => {
                if number > 0 && number < 3 {
                    valid_inv = true;
                    choice = number;
                }
                number
            }
            Err(e) => {
                eprintln!("Wrong number : {e}");
                continue;
            }
        };
    }
    if choice == 1 {
        return Player::new(
            name,
            vec![
                Item {
                    name: String::from("Sword"),
                    item_type: ItemType::Weapon,
                    effect: rand::rng().random_range(5..=25),
                },
                Item {
                    name: String::from("Life potion"),
                    item_type: ItemType::Health,
                    effect: rand::rng().random_range(25..=50),
                },
            ],
        );
    }
    return Player::new(
        name,
        vec![
            Item {
                name: String::from("Poison potion"),
                item_type: ItemType::Weapon,
                effect: rand::rng().random_range(10..=20),
            },
            Item {
                name: String::from("Dodge potion"),
                item_type: ItemType::Weapon,
                effect: rand::rng().random_range(10..=35),
            },
        ],
    );
}

fn play_tour(player: &mut Player, opponent: &mut Player) {
    println!(
        "It's your turn {} ! Choose your option (your health is : {})",
        player.name, player.health
    );
    println!("1. Attack");
    println!("2. Dodge next attack");
    if !player.inventory.is_empty() {
        println!("3. Use an item from your inventory")
    } else {
        println!("Your inventory is empty !")
    }

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");

    let choice = choice.trim();

    match choice {
        "1" => {
            let damages = player.attack();
            if opponent.dodge {
                opponent.dodge = false;
                println!("{} dodged {}'s attack !", opponent.name, player.name);
            } else {
                opponent.health -= damages;
                println!(
                    "{} has attacked {} and made {} damages points",
                    player.name, opponent.name, damages
                );
            }
        }
        "2" => {
            let dodge = player.dodge();
            if dodge {
                player.dodge = true;
                println!("{} will dodge next attack !", player.name);
            } else {
                println!("{} failed to dodge next attack !", player.name);
            }
        }
        _ => {
            eprintln!("Choice not valid");
        }
    }
}

fn main() {
    let mut player1 = create_player_from_stdin(1);
    let mut player2 = create_player_from_stdin(2);
    let mut turn: u32 = 0;

    println!("{player1}");
    println!("{player2}");
    println!();

    while player1.health > 0 && player2.health > 0 {
        if turn % 2 == 0 {
            play_tour(&mut player1, &mut player2);
        } else {
            play_tour(&mut player2, &mut player1);
        }
        turn += 1;
    }
    if player1.health < 0 {
        println!("{} wins !", player2.name)
    } else {
        println!("{} wins !", player1.name)
    }
}

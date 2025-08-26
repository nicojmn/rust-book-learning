use rand::{self, Rng, random_bool};
use std::{fmt, io};

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
            health: 100,
            next_tour: 0,
            inventory: invetory,
        }
    }

    fn attack(&self) -> i32 {
        rand::rng().random_range(0..=15) + self.next_tour
    }

    fn dodge() -> bool {
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
    println!("Enter player {player_number} name :");

    let mut name = String::new();
    let mut choice_inv = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect(&format!("Failed to get player {player_number} name"));

    name = name.trim().to_string();

    loop {
        println!("Choose your inventory : ");
        println!("1. Sword and life potion");
        println!("2. Poison potion and dodge potion");

        io::stdin()
            .read_line(&mut choice_inv)
            .expect("Please enter a valid number");

        if choice_inv == "1" {
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
}

fn main() {
    let player1 = create_player_from_stdin(1);
    let player2 = create_player_from_stdin(2);

    println!("Player {player1}");
    println!("Player {player2}");
}

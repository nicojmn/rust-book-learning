use std::any::TypeId;

use rand::{self, Rng, random_bool};

#[derive(Debug)]
enum ItemType {
    Health,
    Attack,
}

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
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
            inventory: invetory,
        }
    }

    fn attack() -> u32 {
        rand::rng().random_range(0..=15)
    }

    fn dodge() -> bool {
        random_bool(0.5)
    }

    fn use_item(&self, index: usize) -> Result<i32, std::io::Error> {
        if self.inventory.len() > 0 && self.inventory.len() - 1 > index {
            let item = self.inventory.get(index);
        }
        Ok(0)
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

fn main() {
    println!("Hello, world!");
}

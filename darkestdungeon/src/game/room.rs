use std::rc::Rc;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {

    pub fn new(name: &str, contents: Vec<Curio>, wumpus: bool) -> Room {
        Room {name: name.to_string(), contents: contents, halls: Vec::new(), wumpus: wumpus}
    }

    pub fn new_empty() -> Room {
        Room::new("", Vec::new(), false)
    }

    pub fn neighbors_string(&self) -> String {
        let mut string = String::new();
        for hall in &self.halls {
            let room = hall.other(self).clone();
            let ref name = room.borrow().name;
            string.push_str(&name.to_lowercase());
            string.push_str(" ");
        }
        //println!("Neighbors_string: {}", string);
        string
    }

    pub fn wumpus_closeby(&self) -> bool {
        for hall in &self.halls {
            let other = hall.other(self);
            if other.borrow().wumpus {
                return true;
            }
        }
        return false;
    }
}

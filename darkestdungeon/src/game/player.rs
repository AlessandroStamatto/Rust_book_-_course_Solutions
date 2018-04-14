use std;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::curio::Curio;
use super::room::Room;

const MAX_HP: i32 = 25;

pub enum Command {
    Go(String),
    Shoot(String),
}

pub struct Player {
    pub location: Rc<RefCell<Room>>,
    pub hp: i32,
    pub gold: i32,
    won: bool,
}

impl Player {
    pub fn new(location: Rc<RefCell<Room>>) -> Player {
        Player {
            location: location,
            hp: MAX_HP,
            gold: 0,
            won: false,
        }
    }

    pub fn use_curio(&mut self, curio: Curio) {
        match curio {
            Curio::Chest(gold) => {
                println!("You open the chest and gain {} gold.", gold);
                self.gold += gold;
            },
            Curio::SpikeTrap(dmg) => {
                println!("You take {} damage from the spikes.", dmg);
                self.hp -= dmg;
            },
            Curio::Food(heal) => {
                println!("You shove a wall chicken into your gob and heal {} HP.", heal);
                self.hp = std::cmp::min(MAX_HP, self.hp + heal);
            },
            Curio::IronMaiden(sub, dmg) => {
                println!("Dude I love Iron Maiden! This one's pointy, though.");
                println!("You cut yourself on the spikes inside for {} damage.", dmg);
                self.hp -= dmg;
                println!("You open the iron maiden and...");
                self.use_curio(*sub);
            },
            Curio::FallenAdventurer(sub) => {
                println!("You pilfer the corpse and...");
                self.use_curio(*sub);
            },
        }
    }

    /// Execute the given command on the player and board state.
    pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
        Ok(match cmd {
            Command::Go(ref room_name) => {
                self.location = try!(self.find_room(&room_name));
                let curios : Vec<Curio> = self.location.borrow().contents.iter().cloned().collect();
                for curio in curios {
                        self.use_curio(curio);
                } 
                if self.location.borrow().wumpus {
                    println!("You end on the Wumpus Mouth! Devored :(");
                    self.hp = - MAX_HP;
                }
                if self.location.borrow().wumpus_closeby() {
                    println!("");
                    println!("DANGER: you feel the terrible smell of a Wumpus close by...");
                    println!("");
                }
            },
            Command::Shoot(ref room_name) => {
                let room = try!(self.find_room(&room_name));
                let mut room_mut = room.borrow_mut(); 
                if  room_mut.wumpus {
                    println!("");
                    println!("=========== VICTORY ==============");
                    println!("You shoot the Wumpus through the Heart! You won =D");
                    println!("");
                    self.won = true;
                    room_mut.wumpus = false;
                } else {
                    println!("");
                    println!("The arrow falls flat, no Wumpus to be found...");
                    println!("");
                }
            }
        })
    }

    /// Find one of the neighbors of the current room based on its name. Case insensitive.
    fn find_room(&self, room: &str) -> Result<Rc<RefCell<Room>>, ()> {
        let current_room = self.location.borrow();
        //println!("Searching {} in {:?}", room, current_room.neighbors_string().split_whitespace().map(|neigh|neigh.to_string()).collect::<Vec<String>>());
        for (i, name) in current_room.neighbors_string().split_whitespace().enumerate() {
            if name == room {
                return Ok(current_room.halls[i].other(&*current_room));
            }
        }
        Err(())
    }
}

/**/impl fmt::Display for Player {
/**/    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
/**/        write!(f, "You find yourself in {}.\n\nYou have {} HP and {} gold.",
/**/               self.location.borrow().name, self.hp, self.gold)
/**/    }
/**/}

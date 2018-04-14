use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

pub struct Hall {
    pub left: Rc<RefCell<Room>>,
    pub right: Rc<RefCell<Room>>,
}

impl Hall {
    pub fn new() -> Hall {
        Hall {left: Rc::new(RefCell::new(Room::new_empty())), 
              right: Rc::new(RefCell::new(Room::new_empty()))}
    }

    pub fn new_left_right(left: &Rc<RefCell<Room>>, right: &Rc<RefCell<Room>>) -> Hall {
        Hall {left: left.clone(),
              right: right.clone()}
    }

    /// Given a Room `room`, find the room at the other end of Hall `self`.
    pub fn other(&self, room: &Room) -> Rc<RefCell<Room>> {
        match self.left.borrow().name {
            ref name if *name == room.name => self.right.clone(),
            _ => self.left.clone() 
        }
    }

}

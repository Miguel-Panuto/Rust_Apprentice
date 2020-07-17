// This creates the front of house module, with the file name
mod front_of_house;
// And this to load the hosting module
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitList();
    hosting::add_to_waitList();
    hosting::add_to_waitList();
}
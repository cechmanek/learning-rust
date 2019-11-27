//fn serve_order() {} // same name as serving::serve_order(), but it's in a parent module so it's ok

fn fix_incorrect_order() {
    cook_order();
    //super::serve_order(); // calls parent module function, not serving::serve_order()
}

fn cook_order() {}

pub struct Breakfast { // public struct. this can be instantiated from outside
    pub toast: String, // public struct member. this can be set and seen from outside
    seasonal_fruit: String, // private struct member. this can't be set or seen from outside
}

impl Breakfast {
    pub fn summer (toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"), // private members can still be constructed
        }
    }
 }
    
pub enum Appetizer { // if an enum is made public then all of it's members are public
    Soup,
    Salad,
}


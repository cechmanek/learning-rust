mod front_of_house { // declare the 'front_of_house' module
    pub mod hosting { // declare the' front_of_house::hosting' sub module
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving { // declare the' front_of_house::serving' sub module
        fn take_order() {}
        
        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // calls parent module function, not serving::serve_order()
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

}

fn serve_order() {} // same name as serving::serve_order(), but it's in a parent module so it's ok

fn eat_at_restuarant() {
    // call function of module by specifying absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // call function by specifying relative path
    front_of_house::hosting::add_to_waitlist();

    // instantiating structs that have some private members
    
    // Order a breakfast with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // meal is mutable so change our mind about the type of toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast); // toast is public so dot notation works

    // line below will not compile because we can't see private struct members
    //println!("I order the chef to bring me {}", meal.seasonal_fruit);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Soup;
}

// to avoid needing to have such long function names we have the 'use' keyword
use crate::front_of_house::hosting; // now 'hosting' is in scope

pub fn eat_at_diner() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist(); // this function is now in scope

    //hosting::seat_at_table(); // this is in scope, but isn't made public, so still can't be called
}


/* Re-exporting names with 'pub use' syntax
 on line 73 above we have 'use crate::front_of_house::hosting' which brings hosting into scope but
 still doesn't make hosting available outside of this scope. For that we can combine 'pub' and 'use' 
 */
 pub use crate::back_of_house::Appetizer;
 // now back_of_house::Appetizer is available outside this module


struct User
{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn build_user(email: String, username: String) -> User
{
    User {
        email, // shorthand of <email: email> when fn params match struct params
        username, // also shortand for <username: username,>
        sign_in_count: 1,
        active: true,
    }
    // User is return by default when it's the last expression
}


// tuple structs are a way to have typed structs
struct Color(u8, u8, u8);
struct Point(f64, f64, f64);

#[derive(Debug)]
struct Rectangle {
                    width: u32,
                    height: u32,
                 }


// attaching methods to structs is done via an implementation block (impl)
impl Rectangle {
    fn area(&self) -> u32 // can use self or &self it seems?
    {
        return self.width * self.height;
    }

    // you can attach multiple methods in a single implementation block
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        return self.height > other.height && self.width > other.width;
    }

    // associated functions don't act on an instance of a struct, similar to static c++ methods
    fn square(size: u32) -> Rectangle // they don't act on an instance, so don't need 'self'
    {
        return Rectangle{height: size, width: size}
    }
}    

fn main()
{
    // when instantiating structs all fields must be specified
    let user1 = User {
                        email: String::from("gj.cechmanek@gmail.com"),
                        username: String::from("J-Money"),
                        active: true,
                        sign_in_count: 1,
                    };

    println!("Hello, world! My name is {}", user1.username);

    // structs can be declared mutable like regular variables
    let mut user2 = User {
                            email: String::from("grant.jhc@gmail.com"),
                            username: String::from("G-Money"),
                            active: false,
                            sign_in_count: 0,
                        };

    println!("My friend's name is {}", user2.username);
    user2.sign_in_count = 2; // mutating part of the struct
    println!("He's signed in {} times", user2.sign_in_count);

    // structs can be constructed from others via shorthand copy of some values
    let user3 = User {
                        username: String::from("Becca"),
                        email: String::from("becca@gmail.com"),
                        ..user1 // copy all non specified values from user1
                    };

    println!("My GF's  name is {}", user3.username);

    // creating tuple structs
    let green = Color(255, 0, 0);
    let some_point = Point(-3.5, 6.0, 500.0); // must have decimal in float

    // default printing of structs throws an error
    // println!("values of green (bgr) channels: {}", green); // won't compile
    
    // we can try with printing debug settings
    // println!("values of green (bgr) channels: {:?}", green); // won't compile

    // to print structs you have to use the #[derive(Debug)] annotation
    let my_rectangle = Rectangle{width: 12, height: 4};
    println!("the values in our rectagle are: {:?}", my_rectangle); // works
    
    // calling methods of structs. No need for deference pointer operator
    println!("the area of my rectangle is {}", my_rectangle.area());

    // passing other structs to a struct's methods
    let rect1 = Rectangle{height : 20, width : 30};
    let rect2 = Rectangle{height : 10, width : 25};
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    // make a new Rectange via an associated function
    let my_square = Rectangle::square(5);
    println!("my new square has height {}, and area {}", my_square.height, my_square.area());

}

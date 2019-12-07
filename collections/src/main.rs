// working through chapter 8 of The Rust Book: Common Collections

fn main() {

    let v: Vec<i32> = Vec::new(); // empty vector of unknown size. need to specify the type as i32

    // Rust has a convenience function for declaring vectors
    let v2 = vec![1, 2, 3]; // vector of known size. the type is inferred from the initial values

    // can add elements to a vector with the .push() method, but it needs to be mutable
    let mut v3 = Vec::new(); 
    v3.push(5); // this line is where the type of v3 is infered
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // reading elements of vectors
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2]; // create a reference to the third vector element
    println!("the thirds element is {}", third);

    // the get() method returns an option (becuase get(400) may fail) so use match block to evaluate
    match v3.get(2) {
        Some(third) => println!("the thirds element is {}", third),
        None => println!("There is no third element"),
    }

    // because the get() method returns an option we never worry about index out of range errors!
    match v3.get(999999) {
        Some(value_exists) => println!("the value in the vector is {}", value_exists),
        None => println!("That index is out of range element, but Rust didn't crash"),
    }

    // direct reference of non-existant array elements still will crash. which maybe you want it to
    //let out_of_range: &i32 = &v3[5000]; // this line will complile but crash when run

    //iterating over vectors
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    // this can be done with references as above, or with local copies
    for i in v5 {
        println!("{}", i);
    }

    // we can also iterate over mutable vectors
    let mut v6 = vec![100,32, 57];
    for i in &mut v6 {
        println!("{}", i);
        *i += 50; // need to dereference i to get the value it refers to
    }

    // vectors can only hold one type, but variants of enums count as one type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![SpreadsheetCell::Int(3),
                   SpreadsheetCell::Text(String::from("blue")),
                   SpreadsheetCell::Float(10.12)
                  ];

}

fn main() 
{
    let mut x = 5;
    println!("The value of x is {}" ,x);

    x = 6;
    println!("The value of x is {}" ,x);

    // basic compound type is a tuple. fixed size, but can have different sub-types
    let tup: (i32, f64, u8) = ( 500, 6.4, 1);

    // can access tuples via dot notation indexing
    println!("tup values are {}, {}, and {}", tup.0, tup.1, tup.2);
    
    // can also destructure to extract values from a tuple
    let (x,y,z) = tup;
    println!("tup values are {}, {}, and {}", x, y, z);

    // another basic compound type is the array. fixed size, same sub-type
    let prime_array = [2,3,5,7,11]; // type inferenced and size inferenced when initialized

    println!("first prime number is {}", prime_array[0]);

    let empty_array: [u16; 5]; // specify array data type and size, but not initialized
}

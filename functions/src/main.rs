
fn another_function(value: i32) // must declare parameter type in function declaration
{
    println!("The value passd in is {}.", value);
}


fn return_five() -> i32 // specify the return type of this function 
{
    5 // the final line in the function is implicityly returned, but it can't have a semi-colon
}

fn main()
{
    println!("Hello, world!");
    another_function(7);

    let five = return_five();

    another_function(five);
}

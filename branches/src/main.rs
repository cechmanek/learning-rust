

fn main() 
{
    let number = 9;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let answer = return_1_if_odd(number);
    println!("{}", answer);

    looper_function();
    while_looper();
    for_looper();
    fixed_count_for_looper();
}

fn return_1_if_odd(num: i32) -> i32 {
    let condition = num %2 == 0; // type infered as boolean
    
    let x = if condition { // type inference on 'x' to be integer based on arms
        0 // condition == true arm, no semi-colon to set x = 0
    }
    else {
        1 //condition == false arm, no semi-colon to set x = 1
    };
    return x; // explicit return of x can have semi-colon
}

fn looper_function() {
    let mut counter = 0;

    let result = loop { // loops until 'break' is called
        counter += 1;

        if counter == 10 {
            break counter * 2; // loops can have a return value that is specified by 'break'
        }
    };
    println!("The result is {}", result);
}

fn while_looper() {
    let mut number = 3;

    while number != 0 {
        println!("count down {}", number);
        number -= 1;
    }
}

fn for_looper() {
    let arr = [10,20,30,40,50];
    let mut index = 0;

    for element in arr.iter() {
        println!("the value is {}", element);
    }
}

fn fixed_count_for_looper() {
    for number in (1..4) { // this declares a range from 1, 2, 3
        println!("current number is {}", number);
    }
    for number in (1..4).rev() { // this reverses the count to 3, 2, 1
        println!("current number is {}", number);
    }
}
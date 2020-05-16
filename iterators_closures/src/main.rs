use std::thread;
use std::time::Duration;

fn simulated_expensive_calcuation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    return intensity;
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today, do {} pushups!", simulated_expensive_calcuation(intensity));
        println!("Next, do {} situps!", simulated_expensive_calcuation(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today, runfor {} minutes", simulated_expensive_calcuation(intensity));
        }
    }
}

// the same functions as above, but now using expensive_closure instead of expensive_calculation
fn generate_workout_closure(intensity: u32, random_number: u32) {
    // we can define a closure that holds the expensive function call
    let expensive_closure = |num| {// items inside pipes are the parameters passed to the closure
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    };

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure(intensity));
        println!("Next, do {} situps", expensive_closure(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today, run {} minutes!", expensive_closure(intensity));
        }
    }
}

// the local expensive_closure above is a bit better, but still does a costly compute each time

// create a struct the caches the computed value, and only computes it once
struct Cacher<T>
where
    T: Fn(u32) -> u32, // Fn is a trait closures implement.
    // Any closure in Cacher must accept a u32 and return a u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg); // call calculation() function with arg passed in
                self.value = Some(v);
                return v;
            }
        }
    }
}

// now the expensive calculation only happens once and value is stored in expensive_result.value()
// this isn't meaningfully different than creating a class with a method that sets a class attribute
fn generate_workout_better_closure(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new (|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    });

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today, run {} minutes!", expensive_result.value(intensity));
        }
    }

}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

}
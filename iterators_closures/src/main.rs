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

    // below are some examples of using iterators
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val); // here val is type *int, aka only a refernce
    } // for loop implicitly takes ownership, but val still isn't mutable

    // we can't re-use the same iterator, we need to make a new one,
    // but this is free computation, so don't worry about it 
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1)); // .next() returns immutable reference
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3)); 
    assert_eq!(v1_iter.next(), None); // returns None when we get to end of container

    // if we want to take ownership use v1.into_iter()
    let v1_in_iter = v1.into_iter();
    for mut val in v1_in_iter {
        val += 1; // val is now type int
    }

    // if we want mutable references us v1.iter_mut(), just remember that container must be mutable
    let mut v2 = vec![1,2,3];
    let v2_it_mut = v2.iter_mut();
    for val in v2_it_mut {
        *val = *val  + 1; // val is a reference, so need to dereference with *
    }

    // iterators get used up when called, but only when consumed by something
    let v3 = vec![1,2,3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum(); // sum() is a collector that consumes an iterator
    println!("the sum of [1,2,3] is {}",total);

    // another collection example
    let v4 = vec![1,2,3];
    let v4_iter = v4.iter();
    let plus1: Vec<_> = v4_iter.map(|x| x+1).collect();
    // map() returns another iterator, collect() is a collector that consumes an iterator
    assert_eq!(plus1, vec![2,3,4]);

    // yet another collection, filters
    let v5 = vec![1,2,3,4,5,6,7,8,9,10];
    let v5_iter = v5.into_iter(); // use into_iter so we don't have to dereference
    let evens: Vec<_> = v5_iter.filter(|x| (x%2)==0).collect();
    assert_eq!(evens, vec![2,4,6,8,10]);

    // use our custom Counter class with its iterator methods
}

// to create our own iterator we need to implement the iterator trait, which only needs .next()
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        else {
            return None;
        }
    }

}
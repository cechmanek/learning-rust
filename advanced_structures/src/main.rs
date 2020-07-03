fn main() {
  // example of unsafe rust super powers: dereferencing raw pointers  
  let mut num = 5;

  // create raw pointer by casting regular Rust pointers to their raw types
  let r1 = &num as *const i32; // *const is the full name, the asterisc is part of type name
  let r2 = &mut num as *mut i32; // *mut is full name 

  // example of raw memory access
  let address = 0x012345usize; // some random hardcoded access
  let r = address as *const i32;

  unsafe { // can only dereference these r1 r2 pointers inside an unsafe{} block
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }

  unsafe {
    dangerous(); // can only be called inside an unsafe{} code block
  }

  // using external code must be done inside an unsafe block
  unsafe {
    let positive_five = abs(-5); // call the "C" abs() function
    println!("calling abs() from C works: abs(-5) ==> {}",positive_five);
  }
}

// defining unsafe functions is identical to regular functions, except with the unsafe keyword
unsafe fn dangerous() {
  // do some unsafe actions. no need to add an usafe{} block. this whole function is an unsafe block
}

// we can have regular, safe, functions that have internal unsafe{} blocks
fn split_at_index(my_arr: &mut [i32], mid: usize) ->(&mut[i32], &mut[i32]) {
  // accept an array and index, return two arrays split at the index
  let len = my_arr.len();

  assert!( mid <= len);
  // return (&mut slice[..mid], &mut slice[mid..]); // won't compile as we have 2 borrows
  // even though we as programers know the mutable slices don't overlap, the compiler doesn't

  use std::slice;
  let ptr = my_arr.as_mut_ptr(); // raw pointer. we can access safely, but can't dereference
  unsafe {
    let a = (slice::from_raw_parts_mut(ptr, mid),
             slice::from_raw_parts_mut(ptr.add(mid), len - mid)
            );
    return a;
  }
}

// we can call code from other languages in Rust. this is always unsafe
extern "C" {
  fn abs(input: i32) -> i32;
}

// we can define Rust as callable from other languages
#[no_mangle] // tell Rust compiler to not modify this function name
pub extern "C" fn call_from_c() {
  println!("Just called this Rust function from C!");
}
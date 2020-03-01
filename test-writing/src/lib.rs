
// some methods to test
pub fn add_two(a: i32) -> i32 {
    return a + 2;
}


#[cfg(test)]
mod tests {
    use super::*; // in order to see the add_two(..) function we need include it in this mod

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn it_still_works() {
        assert_ne!(2 + 2, 5);
    }
    #[test]
    fn exploration() {
        assert!(3*3 == 9);
    }

    #[test]
    fn meant_to_fail() {
        panic!("Make this test fail");
    }
    
    #[test]
    fn do_nothing() { // can't panic so it passes
    }

    fn not_a_test() {
        println!("not called because it doesn't have #[test]");
    }

    #[test]
    fn it_adds_two() { // test an external function
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn custom_failure_message() {
        let sentence: String = String::from("My name is Steve.");
        assert!(sentence.contains("Steven"),
                "Failure. sentence ' {} ' does not cotain Steven", sentence);
    }

    #[test]
    #[should_panic] // we can test for proper error handling by using [should_panic]
    fn divide_by_zero() {        
        let zero: i32 = 0;
        let bad_value: i32 = 15 / zero; // this line is supposed to panic
        assert!(bad_value==15); // don't need assert here. this test will pass if bad_value panics
    }

    #[test]
    #[ignore] // this test isn't run by normal 'cargo test' command instructions 
    fn super_slow_test() {
        // use 'cargo test super_slow_test' to run just this, or cargo test -- --ignored
        // yes, there are two sets of double dashes
        for i in 0..999999 {
            // do some slow math
        }
        assert!(1 == 1);
    }
}


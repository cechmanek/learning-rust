fn main()
{
    // we can manually throw a runtime error (aka panic) with the panic! macro
    //panic!("crash and burn!");

    // panic! also gets thrown for other errors, like index out of bounds errors
    let v = vec![1,2,3];
    // v[99]; // cause a runtime error
    // if you look at the stack trace is says that panic! is thrown by libcore/slice/mod.rs:2715:10

    // rather than use try-except or try-catch blocks, Rust uses the Result<T, E> enum type.
    // a common place for this is in trying to open files that may not exist

    use std::fs::File;
    let my_file = File::open("hello.txt"); // 'hello.txt' doesn't exist
    // 'f' is a result enum type that could be 'OK(T)' or 'Err(E)'
    // a handy way to see what type a function returns is to intentionally assign it a wrong type,
    // and then see what error the compiler throws. ex:
    // let f:u32 = File::open("hello.txt"); // 'f' is definitely not an u32 unsigned int
    
    // now that we have a Result enum we can use a match block
    let my_file = match my_file // using shadowing on 'my_file' 
    { 
        Ok(file) => file,
        Err(error) => { panic!("Problem opening file: {:?}", error) },
    };
    
    
    // we can nest match blocks to do different things based on the type or Result==Err(E) enum
    use std::io::ErrorKind;
    let other_file = File::open("hello.txt");
    let other_file = match other_file 
    {
        Ok(file) => file, // return file handle if we opened it just fine
        Err(error) => match error.kind()
        {
            ErrorKind::NotFound => match File::create("hello.txt") // make file if it doesn't exist
            {
                Ok(file_created) => file_created,
                Err(e) => panic!("Problem creating file {:?}", e),
            },

            // if we failed for any reason other than ErrorKind::NotFound then panic
            other_error => panic!("Problem opening the file {:?}, other than ErrorKind::NotFound", other_error),
        },
    };

    // we can use the unwrap() method on Result<T,E> as a shorthand for match blocks
    // it automatically extracts the success or Error
    //let my_file = File::open("hello.txt").unwrap(); // panics as 'hello.txt' doesn't exist

    // we can customize error messages in panics by using the expect() method instead
    let my_file = File::open("hello2.txt").expect("Failed to open hello2.txt");
    // this is identical to .unwrap() except you can pass a custom message


    // Propogating Errors
    // you can return errors from functions so that parent code can better handle them
    use std::io;
    use std::io::Read;
    
    fn read_user_name_from_file() -> Result<String, io::Error>
    {
        let my_file = File::open("hello3.txt");

        let mut my_file = match my_file {
            Ok(file) => file,
            Err(e) => return Err(e), // if we can't open the file for any reason return the error to parent code
        };

        let mut my_string = String::new();

        //match my_file.read_to_string(&mut my_string) {
        //   Ok(_) => Ok(my_string),
        //   Err(e) => Err(e),
        //},
        
        // this last match() block is infuriating!
        // Notice the comma terminating the match block instead of a semicolon. Literally the smallest possible character change between ',' and ';' 
        // that comma indicates that this whole function block returns either Ok(my_string), or Err(e), which correspond to Result<String, io::Error>
        // Expecting people to notice ',' vs ';' instead of using the wonderful keyword 'return' is a disgraceful crime commited by the Rust core team!

        // an more intelligent way to do this would be:
        return match my_file.read_to_string(&mut my_string) {
            Ok(_) => Ok(my_string),
            Err(e) => Err(e),
        };
    }

    // Rust has some fucked up bullshit shorthand that replaces match blocks that contain "Err(e) => return Err(e)" with a ? instead
    // it's total garbage and should never be used, so I won't reproduce it here.

}

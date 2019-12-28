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

    // got up to "Listing 9-5: Handling different kinds of errors in different ways" 


}

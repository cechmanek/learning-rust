fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[0..s.len()];
}


fn main() {
   let mut my_string = String::from("Hello, world!");

   println!("first word is {}", get_first_word(&my_string));

    let first_word = get_first_word(&my_string);

    // what happens to first_word after we mutate my_string?

    // can only run of of the two lines below. having both throws a compile error
    my_string.clear();
    //println!("what's first_word now?? {}", first_word);

}

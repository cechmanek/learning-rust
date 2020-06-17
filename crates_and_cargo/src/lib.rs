//! This is a top level crate comment
//! 
//! the crate named 'crates_and_cargo' is created by Justin Cechmanek as a way to learn about Rust.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crates_and_cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// 
/// Due to the terrifying incompetence of the Rust language developers this above example (the one
/// that is FULLY COMMMENTED OUT!!!!!) is found and executed as actual code by 'cargo test'.
/// SHOCKINGLY DUMB!
pub fn add_one(x: i32) -> i32 {
  return x + 1;
}
/// A function defined in lib.rs 
/// 
/// This is a documenation comment that is annotated with 3 backslashes '///'
/// 
/// It supports `Markdown`
///  
/// run >> cargo doc to generate html documentation
/// # like this heading
/// The headings are links to something 
/// ## and this sub heading
///
///  
/// run >> cargo doc --open to view the html in your browswer
pub fn view_doc(x: i32) ->bool {
  let view_documentation = true;
  return view_documentation;
}
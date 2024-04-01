// macros are a way of writing code that writes other code, which is known as metaprogramming. 
// A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters:
// Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.
// Two types of macros: declarative macros and procedural macros

// ====================
// DECLARATIVE MACROS
// ====================
// simplified version of the vec! macro
#[macro_export]       // This attribute makes the macro available to other crates when this crate is used as a dependency.
macro_rules! vec {    // The macro_rules! keyword indicates that we’re defining a macro. - the body is similar to a match expression
    ( $( $x:expr ),* ) => {
      // The pattern $x:expr matches any expression.
      // * specifies that the pattern matches zero or more expressions and that this pattern can repeat.
        {
            let mut temp_vec = Vec::new();
            $(   
                temp_vec.push($x); // generates code for each expression match
            )*
            temp_vec
        }
    };
}

// ====================
// PROCEDURAL MACROS
// ====================
// procedural macro, which acts more like a function (and is a type of procedure). 
// Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do
// The three kinds of procedural macros: custom derive macros, attribute-like macros, and function-like macros.
// When creating procedural macros, the definitions must reside in their own crate with a special crate type. This is for complex technical reasons that we hope to eliminate in the future.

// Syntax -
// use proc_macro;
// #[some_attribute]
//   pub fn some_name(input: TokenStream) -> TokenStream { 
// }

// If needed see code in rust lang book - currently would need to create a new crate for this

// Crazy naming convention for the crate
// Also need to update Cargo.toml of hello_macro_derive. Add these
// [lib]
// proc-macro = true

// [dependencies]
// syn = "1.0"
// quote = "1.0"

// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;
// #[derive(HelloMacro)]
struct Pancakes;

pub fn macros() {
  // Declarative macros
  let v: Vec<u32> = vec![1, 2, 3];

  // Pancakes::hello_macro();

}

pub mod ser;
pub mod utils;


// // src/lib.rs
// extern crate macro_lib;

// use macro_lib::*;

// extern crate serde;


// Example: Basic function
// #[derive(Attach)]
// #[attach(foo(SomeType, OtherType))]
// #[attach(bar(OtherType))]
// struct NameTree {

// }
// out: attr: ""
// out: item: "fn invoke1() { }"

// // Example: Attribute with input
// #[show_streams(bar)]
// fn invoke2() {}
// // out: attr: "bar"
// // out: item: "fn invoke2() {}"

// // Example: Multiple tokens in the input
// #[show_streams(multiple => tokens)]
// fn invoke3() {}
// // out: attr: "multiple => tokens"
// // out: item: "fn invoke3() {}"

// // Example:
// #[show_streams { delimiters }]
// fn invoke4() {}
// // out: attr: "delimiters"
// // out: item: "fn invoke4() {}"
// mod main_ser;
// mod main_regex;
// mod main_sample;
// mod main_syn;
mod main_test;
fn main() {
    // invoke1();
    // invoke2();
    // invoke3();
    // invoke4();
    // main_ser::main();
    // main_regex::main();
    // main_sample::main();
    // main_syn::main();
    main_test::main();
    // let s = "hello !";

}

fn _foo() {

}

#[derive(Debug)]
pub struct _Local {
    pub _attrs: String,
    pub pat: String,

}

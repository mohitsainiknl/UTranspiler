


// pub fn main() {
//     // use regex_syntax::Parser;
//     use regex_syntax::ast::parse::Parser;

//     // use regex_syntax::hir::{self, Hir};

//     let hir = Parser::new().parse_with_comments("(?#test),(?#text)").unwrap();
//     // assert_eq!(hir, Hir::alternation(vec![
//     //     Hir::literal(hir::Literal::Unicode('a')),
//     //     Hir::literal(hir::Literal::Unicode('b')),
//     // ]));
//     println!("{:#?}", hir);
// }
// include the latest version of the regex crate in your Cargo.toml
extern crate fancy_regex;

use fancy_regex::Regex;

pub fn main() {
  let regex = Regex::new(r"(\((?:[^()]|(?R))+\))|([^\(\)]+)").unwrap();
  let string = "gj(simple)(?#and(nested)) b$%\\\\ (jh((h(h))))";
  
  // result will be an iterator over tuples containing the start and end indices for each match in the string
  let result = regex.captures_iter(string);
  
  for mat in result {
    println!("{:?}", mat);
  }
}

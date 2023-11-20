
use regex_syntax::ast::parse::Parser;


#[allow(unused)]
pub fn main() {
    let ast = Parser::new().parse("(a|b|c)+").unwrap();

    println!("{:?}", ast);
}
mod lexer;
use lexer::lexer;

fn main() {
    let l = lexer("12 + (3 - 123) * 3 / 4").unwrap();
    println!("{:?}", l);
}
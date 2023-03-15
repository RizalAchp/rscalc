pub mod error;
pub mod tok;
pub mod parser;
pub mod value;
pub mod lev;



#[test]
fn test_parse() {
    let mut expressions = parser::parse("(2 * 2 + 4_822) / 4").unwrap();
    println!("{expressions:#?}");

    let evaluate = expressions.eval().unwrap();
    println!("evalueate to {evaluate}");

    let mut expressions = parser::parse("1 == 1").unwrap();
    println!("{expressions:#?}");

    let evaluate = expressions.eval().unwrap();
    println!("evalueate to {evaluate}");
}

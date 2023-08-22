use rscalc::*;

fn main() {
    let t = "4.20 * ((69 + 69_420) >> 69)";

    println!("Cmd: {t}");
    let mut lexer = Lexer::new(t);
    println!("Lex: {lexer:?}");

    'l: loop {
        if let Some(lex) = lexer.next() {
            match lex {
                Ok(x) => {
                    println!("{x:?},");
                }
                Err(err) => {
                    eprintln!("{err}");
                    break 'l;
                }
            }
        } else {
            break 'l;
        }
    }
}

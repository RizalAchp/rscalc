use super::{Lexer, Token};

fn test_tokens<'a>(source: &'a str, expect: Vec<Option<Token<'a>>>) {
    let lexer = Lexer::new(source);
    let lex_tokens: Vec<_> = lexer
        .map(|x| match x {
            Ok(ok) => Some(ok),
            Err(err) => {
                eprintln!("{err}");
                None
            }
        })
        .collect();

    for (idx, (tok, exp)) in lex_tokens.into_iter().zip(expect.into_iter()).enumerate() {
        assert_eq!(tok, exp, "on_index: {idx}");
    }
}

#[test]
fn simple() {
    test_tokens(
        "64 * 420",
        vec![
            Some(Token::Int(64)),
            Some(Token::Mul),
            Some(Token::Int(420)),
        ],
    )
}

#[test]
fn numerals() {
    test_tokens(
        r#"
0xdeadbeef 12345 12345. 3.1415e-2 0.123E-10 1.7976931348623157E+500 9223372036854775807 9223372036854775808.
"#,
        vec![
            Some(Token::Int(0xdeadbeef)),
            Some(Token::Int(12345)),
            Some(Token::Float(12345.0)),
            Some(Token::Float(3.1415e-2)),
            Some(Token::Float(0.123e-10)),
            Some(Token::Float(f64::INFINITY)),
            Some(Token::Int(9223372036854775807)),
            Some(Token::Float(9223372036854775808.0)),
        ],
    );
}
#[test]
fn ops() {
    test_tokens(
        r#"
- -= + += * / *= /= // //= ^ ^= % %= & &= ~ ~= ^^ ^^= | ,
|= ; >> >>= << <<= . .. ..= = < <= > >= == != : , :: ( ) 
[ ] { } := :: && || -> => 
            "#,
        vec![
            Some(Token::Sub),
            Some(Token::SubAssign),
            Some(Token::Add),
            Some(Token::AddAssign),
            Some(Token::Mul),
            Some(Token::Div),
            Some(Token::MulAssign),
            Some(Token::DivAssign),
            Some(Token::IDiv),
            Some(Token::IDivAssign),
            Some(Token::Pow),
            Some(Token::PowAssign),
            Some(Token::Rem),
            Some(Token::RemAssign),
            Some(Token::BitAnd),
            Some(Token::BitAndAssign),
            Some(Token::BitNotXor),
            Some(Token::BitNotXorAssign),
            Some(Token::BitXor),
            Some(Token::BitXorAssign),
            Some(Token::BitOr),
            Some(Token::Comma),
            Some(Token::BitOrAssign),
            Some(Token::SemiColon),
            Some(Token::Shr),
            Some(Token::ShrAssign),
            Some(Token::Shl),
            Some(Token::ShlAssign),
            Some(Token::Dot),
            Some(Token::Range),
            Some(Token::RangeInc),
            Some(Token::Assign),
            Some(Token::Lt),
            Some(Token::Lte),
            Some(Token::Gt),
            Some(Token::Gte),
            Some(Token::Eq),
            Some(Token::Ne),
            Some(Token::Colon),
            Some(Token::Comma),
            Some(Token::Decl),
            Some(Token::LParen),
            Some(Token::RParen),
            Some(Token::LBracket),
            Some(Token::RBracket),
            Some(Token::LSquirly),
            Some(Token::RSquirly),
            Some(Token::DeclAssign),
            Some(Token::Decl),
            Some(Token::LogicAnd),
            Some(Token::LogicOr),
            Some(Token::Arrow),
            Some(Token::FatArrow),
        ],
    );
}

#[test]
fn words() {
    test_tokens(
        r#"
            break else elif if for while return true false custom names name123num
        "#,
        vec![
            Some(Token::Break),
            Some(Token::Else),
            Some(Token::Elif),
            Some(Token::If),
            Some(Token::For),
            Some(Token::While),
            Some(Token::Return),
            Some(Token::True),
            Some(Token::False),
            Some(Token::ident("custom".to_owned())),
            Some(Token::ident("names".to_owned())),
            Some(Token::ident("name123num".to_owned())),
        ],
    );
}

#[inline]
fn ident(i: &str) -> Token<'_> {
    Token::ident(i.to_owned())
}

#[test]
fn code() {
    test_tokens(
        r#"
MyStruct :: struct {
   name  :string;
   age   :i32;
   hight :i32;
}
my_function :: () -> int {
   1 + 1
}
CONSTANT_VAR :: 69;
main :: () {
   new_mutable_var := 123;
   new_explicit_type :f32= 123.0; 

   new_mutable_var = 420;
   new_explicit_type = 6.9;

   println(CONSTANT_VAR);

   mystrc := MyStruct{};
   println(mystrc);

   println(my_function());
}
    "#,
        vec![
            Some(ident("MyStruct")),
            Some(Token::Decl),
            Some(Token::Struct),
            Some(Token::LSquirly),
            Some(ident("name")),
            Some(Token::Colon),
            Some(ident("string")),
            Some(Token::SemiColon),
            Some(ident("age")),
            Some(Token::Colon),
            Some(ident("i32")),
            Some(Token::SemiColon),
            Some(ident("hight")),
            Some(Token::Colon),
            Some(ident("i32")),
            Some(Token::SemiColon),
            Some(Token::RSquirly),
            Some(ident("my_function")),
            Some(Token::Decl),
            Some(Token::LParen),
            Some(Token::RParen),
            Some(Token::Arrow),
            Some(ident("int")),
            Some(Token::LSquirly),
            Some(Token::Int(1)),
            Some(Token::Add),
            Some(Token::Int(1)),
            Some(Token::RSquirly),
            Some(ident("CONSTANT_VAR")),
            Some(Token::Decl),
            Some(Token::Int(69)),
            Some(Token::SemiColon),
            Some(ident("main")),
            Some(Token::Decl),
            Some(Token::LParen),
            Some(Token::RParen),
            Some(Token::LSquirly),
            Some(ident("new_mutable_var")),
            Some(Token::DeclAssign),
            Some(Token::Int(123)),
            Some(Token::SemiColon),
            Some(ident("new_explicit_type")),
            Some(Token::Colon),
            Some(ident("f32")),
            Some(Token::Assign),
            Some(Token::Float(123.0)),
            Some(Token::SemiColon),
            Some(ident("new_mutable_var")),
            Some(Token::Assign),
            Some(Token::Int(420)),
            Some(Token::SemiColon),
            Some(ident("new_explicit_type")),
            Some(Token::Assign),
            Some(Token::Float(6.9)),
            Some(Token::SemiColon),
            Some(ident("println")),
            Some(Token::LParen),
            Some(ident("CONSTANT_VAR")),
            Some(Token::RParen),
            Some(Token::SemiColon),
            Some(ident("mystrc")),
            Some(Token::DeclAssign),
            Some(ident("MyStruct")),
            Some(Token::LSquirly),
            Some(Token::RSquirly),
            Some(Token::SemiColon),
            Some(ident("println")),
            Some(Token::LParen),
            Some(ident("mystrc")),
            Some(Token::RParen),
            Some(Token::SemiColon),
            Some(ident("println")),
            Some(Token::LParen),
            Some(ident("my_function")),
            Some(Token::LParen),
            Some(Token::RParen),
            Some(Token::RParen),
            Some(Token::SemiColon),
            Some(Token::RSquirly),
        ],
    )
}

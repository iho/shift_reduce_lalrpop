pub mod tokens;
pub mod ast;

extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator1);


fn parse_expr(expr : &str) -> String {
    format!("{}", calculator1::exprParser::new().parse(expr).unwrap())
}
fn main() {

    assert_eq!("1", parse_expr("1"));
    assert_eq!("(++1)", parse_expr("++1"));
    assert_eq!("(++1)", parse_expr("++(1)"));
    assert_eq!("(++1)", parse_expr("++ 1"));
    assert_eq!("(++1)", parse_expr("++ (1)"));


    assert_eq!("((1 + 2) + 3)", parse_expr("1 + 2 + 3"));
    assert_eq!("((1 - 2) - 3)", parse_expr("1 - 2 - 3"));
    assert_eq!("((1 * 2) * 3)", parse_expr("1 * 2 * 3"));
    assert_eq!("((1 / 2) / 3)", parse_expr("1 / 2 / 3"));

    assert_eq!("(1 + (2 / 3))", parse_expr("1 + 2 / 3"));
    assert_eq!("(1 + (2 * 3))", parse_expr("1 + 2 * 3"));
    assert_eq!("(1 - (2 / 3))", parse_expr("1 - 2 / 3"));
    assert_eq!("(1 - (2 * 3))", parse_expr("1 - 2 * 3"));

    assert_eq!("((1 / 2) + 3)", parse_expr("1 / 2 + 3"));
    assert_eq!("((1 * 2) + 3)", parse_expr("1 * 2 + 3"));
    assert_eq!("((1 / 2) - 3)", parse_expr("1 / 2 - 3"));
    assert_eq!("((1 * 2) - 3)", parse_expr("1 * 2 - 3"));

    assert_eq!("(((1 / 2) + 3) == (1 + (2 * 3)))", parse_expr("1 / 2 + 3 == 1 + 2 * 3"));

    assert!(calculator1::exprParser::new().parse("1 == 1").is_ok());
    assert!(calculator1::exprParser::new().parse("((1 / 2) + 3) == ((1 * 2) + 3)").is_ok());
    assert!(calculator1::exprParser::new().parse("((1 + 2) + 3) == ((1 - 2) + 3)").is_ok());
    assert!(calculator1::exprParser::new().parse("1 == 1 == 1").is_err());

    // testing ++
    assert_eq!("(((1 + (++2)) + ((3 * 4) * (++5))) + 6)", parse_expr("1 + ++2 + 3 * 4 * ++5 + 6"));
    assert_eq!("((++(1 + (++2))) * 3)", parse_expr("++(1 + ++2) * 3"));
    assert_eq!("((++(1 + (++2))) * 3)", parse_expr("++ (1 + ++2) * 3"));
}

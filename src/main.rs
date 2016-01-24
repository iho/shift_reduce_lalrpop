pub mod calculator1; // synthesized by LALRPOP
pub mod tokens;
pub mod ast;

fn parse_expr(expr : &str) -> String {
    format!("{}", calculator1::parse_expr(expr).unwrap())
}

fn main() {

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
    assert_eq!("((((1 / 2) + 3) == (1 + (2 * 3))) == (1 + (2 * 3)))", parse_expr("1 / 2 + 3 == 1 + 2 * 3 == 1 + 2 * 3"));

    // testing ++
    assert_eq!("(((((1 + (++2)) + ((3 * 4) * (++5))) + 6) == ((23 * 2) + 1)) == 45)", parse_expr("1 + ++2 + 3 * 4 * ++5 + 6 == 23 * 2 + 1 == 45"));
    assert_eq!("((++(1 + (++2))) * 3)", parse_expr("++(1 + ++2) * 3"));
}

mod lexer;
mod s_expr;
use crate::parser::expression_parser::{
    lexer::{Lexer, Token},
    s_expr::S,
};

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        Token::Op(op) => {
            let ((), r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp);
            S::Cons(op, vec![rhs])
        }
        t => panic!("Bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();
            lhs = if op == '[' {
                let rhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(']'));
                S::Cons(op, vec![lhs, rhs])
            } else {
                S::Cons(op, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();
            let rhs = expr_bp(lexer, r_bp);

            lhs = S::Cons(op, vec![lhs, rhs]);
            continue;
        }
        break;
    }

    lhs
}

fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '+' | '-' => ((), 5),
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let res = match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        '.' => (10, 9),
        _ => return None,
    };
    Some(res)
}

fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    match op {
        '!' => Some((7, ())),
        _ => None,
    }
}

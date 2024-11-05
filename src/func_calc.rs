use core::f64;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Sqrt(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Asin(Box<Expr>),
    Acos(Box<Expr>),
    Atan(Box<Expr>),
    ToRadian(Box<Expr>),
    ToDegree(Box<Expr>),
    Pi,
    E,
}

impl Expr {
    fn eval(&self) -> f64 {
        // enum Exprをf64型の値として評価する
        match self {
            Expr::Number(data) => *data,
            Expr::Add(left, right) => left.eval() + right.eval(),
            Expr::Subtract(left, right) => left.eval() - right.eval(),
            Expr::Multiply(left, right) => left.eval() * right.eval(),
            Expr::Divide(left, right) => left.eval() / right.eval(),
            Expr::Pow(left, right) => left.eval().powf(right.eval()),
            Expr::Sqrt(data) => data.eval().sqrt(),
            Expr::Sin(data) => data.eval().sin(),
            Expr::Cos(data) => data.eval().cos(),
            Expr::Tan(data) => data.eval().tan(),
            Expr::Asin(data) => data.eval().asin(),
            Expr::Acos(data) => data.eval().acos(),
            Expr::Atan(data) => data.eval().atan(),
            Expr::ToRadian(data) => data.eval() / 180.0 * Expr::Pi.eval(),
            Expr::ToDegree(data) => data.eval() / Expr::Pi.eval() * 180.0,
            Expr::Pi => f64::consts::PI,
            Expr::E => f64::consts::E,
        }
    }
}

pub fn calc(expression: &str) -> Result<f64, String> {
    // 余分な空白を削除
    let bindstring = expression.replace(" ", "");
    match stparse(&bindstring) {
        Ok(data) => Ok(data.eval()),
        Err(e) => Err(e),
    }
}

fn stparse(inputdata: &str) -> Result<Expr, String> {
    // 文字列をExpr型にparseする
    // 消費しないことを選択できるitertorを作成
    let mut chars = inputdata.chars().peekable();
    parse_expression(&mut chars)
}

fn parse_expression(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    // Charsを受け取り、四則演算式に基づいて組み替えたExprを返す
    // 優先度の高い計算はparse_termで処理
    let mut left = parse_term(chars)?;
    loop {
        match chars.peek() {
            Some('+') => {
                chars.next();
                let right = parse_term(chars)?;
                left = Expr::Add(Box::new(left), Box::new(right));
            }
            Some('-') => {
                chars.next();
                let right = parse_term(chars)?;
                left = Expr::Subtract(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_term(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    // 優先度の高い計算を処理
    // 数字などの要素の評価は優先的に処理
    let mut left = parse_fanctor(chars)?;
    loop {
        match chars.peek() {
            Some('*') => {
                chars.next();
                let right = parse_fanctor(chars)?;
                left = Expr::Multiply(Box::new(left), Box::new(right));
            }
            Some('/') => {
                chars.next();
                let right = parse_fanctor(chars)?;
                left = Expr::Divide(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    Ok(left)
}
fn parse_fanctor(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    // 数字や関数など要素単位parse処理
    let mut base = match chars.peek() {
        // 括弧の処理
        Some(&'(') => {
            // "("を消費
            chars.next();
            let expr = parse_expression(chars)?;
            match chars.next() {
                Some(')') => expr,
                _ => return Err("Expected closing parenthesis".to_string()),
            }
        }
        // 関数の処理
        Some(&c) if c.is_alphabetic() => match parse_function(chars) {
            Ok(data) => data,
            Err(e) => return Err(e),
        },
        // 数字のparse処理
        Some(&c) if c.is_ascii_digit() || c == '.' => {
            let mut number_str = "".to_string();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() || c == '.' {
                    number_str += c.to_string().as_str();
                    chars.next();
                } else {
                    break;
                }
            }
            match number_str.parse::<f64>() {
                Ok(n) => Expr::Number(n),
                Err(e) => return Err(e.to_string()),
            }
        }

        _ => return Err("Unexpected character".to_string()),
    };
    // 指数演算子’^’の処理
    if let Some(&'^') = chars.peek() {
        chars.next();
        let right = parse_fanctor(chars)?;
        base = Expr::Pow(Box::new(base), Box::new(right));
    }
    Ok(base)
}

fn parse_function(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    let mut function_name = "".to_string();
    while let Some(&c) = chars.peek() {
        if c.is_alphabetic() {
            function_name += c.to_lowercase().to_string().as_str();
            chars.next();
        } else if c == '(' {
            chars.next();
            break;
        } else {
            return Err(format!("Expected parethesis after {}", function_name));
        }
    }
    match function_name.as_str() {
        "sqrt" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::Sqrt(Box::new(arg))),
                _ => Err("Expected closing parenthesis after sqrt".to_string()),
            }
        }
        "sin" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::Sin(Box::new(arg))),
                _ => Err("Expected closing parenthesis after sin".to_string()),
            }
        }
        "cos" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::Cos(Box::new(arg))),
                _ => Err("Expected closing parenthesis after cos".to_string()),
            }
        }
        "tan" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::Tan(Box::new(arg))),
                _ => Err("Expected closing parenthesis after tan".to_string()),
            }
        }
        "asin" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::Asin(Box::new(arg))),
                _ => Err("Expected closing parenthesis after asin".to_string()),
            }
        }
        "acos" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::Acos(Box::new(arg))),
                _ => Err("Expected closing parenthesis after acos".to_string()),
            }
        }
        "atan" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::Atan(Box::new(arg))),
                _ => Err("Expected closing parenthesis after atan".to_string()),
            }
        }
        "torad" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::ToRadian(Box::new(arg))),
                _ => Err("Expected closing parenthesis after torad".to_string()),
            }
        }
        "todegree" => {
            let arg = parse_expression(chars)?;
            match chars.next() {
                Some(')') => Ok(Expr::ToDegree(Box::new(arg))),
                _ => Err("expected closing parenthesis after todegree".to_string()),
            }
        }
        "pi" => match chars.next() {
            Some(')') => Ok(Expr::Pi),
            _ => Err("Expected closing parenthesis after pi".to_string()),
        },
        "e" => match chars.next() {
            Some(')') => Ok(Expr::E),
            _ => Err("Expected closing parenthesis after e".to_string()),
        },
        _ => Err(format!("Unknown function: {}", function_name)),
    }
}
#[test]
fn calctest() {
    let inner_parse = |x: &str| calc(x).unwrap();
    assert_eq!(inner_parse("1+1.5"), 2.5);
    assert_eq!(inner_parse("1 +  1.5"), 2.5);
    assert_eq!(inner_parse("32 -23"), 9.0);
    assert_eq!(inner_parse("tan(pi()/3)"), (f64::consts::PI / 3.0).tan());
    assert_eq!(inner_parse("cos(pi()/3)"), (f64::consts::PI / 3.0).cos());
    assert_eq!(inner_parse("sin(pi()/3)"), (f64::consts::PI / 3.0).sin());
    assert_eq!(
        inner_parse("2+4.5*sin(pi()/3)"),
        2.0 + 4.5 * (f64::consts::PI / 3.0).sin()
    );
    assert_eq!(
        inner_parse("2+2*3*(2.4+3)+5/2"),
        2.0 + 2.0 * 3.0 * (2.4 + 3.0) + 5.0 / 2.0
    );
    assert_eq!(
        inner_parse("2+4.5*sin(pi()/2)"),
        2.0 + 4.5 * (f64::consts::PI / 2.0).sin()
    );
    assert_eq!(inner_parse("todegree(asin(sin(torad(60))))"), 60.0);
    assert_eq!(inner_parse("todegree(acos(cos(torad(60))))"), 60.0);
    assert_eq!(inner_parse("todegree(atan(tan(torad(60))))"), 60.0);
    assert_eq!(inner_parse("e()"), f64::consts::E);
    assert_eq!(inner_parse("pi()"), f64::consts::PI);
    assert_eq!(inner_parse("sqrt(4)+sqrt(9)"), 5.0);
    assert_eq!(inner_parse("sqrt(4)*sqrt(9)"), 6.0);
    assert_eq!(inner_parse("3^2+2^3"), 17.0);
    assert_eq!(inner_parse("3^2/sqrt(9)"), 3.0);
}

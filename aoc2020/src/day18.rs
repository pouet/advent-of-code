#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Number(isize),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Par(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Mul,
    LeftPar,
    RightPar,
    Digit(char),
    Eos,
}

#[derive(PartialEq)]
pub enum AddPrecedence {
    Equal,
    Before
}

pub struct ExprParser {
    s: String,
    current: usize,
    add_prec: AddPrecedence
}

impl ExprParser {
    fn new(s: &str, add_prec: AddPrecedence) -> ExprParser {
        ExprParser {
            s: s.to_string(),
            current: 0,
            add_prec
        }
    }

    fn next_token(&mut self) -> Token {
        match self.s.chars().nth(self.current) {
            Some('+') => Token::Plus,
            Some('*') => Token::Mul,
            Some('(') => Token::LeftPar,
            Some(')') => Token::RightPar,
            Some(n @ '0'..='9') => Token::Digit(n),
            Some(' ') => {
                self.consume();
                self.next_token()
            }
            None => Token::Eos,
            Some(c) => panic!("Invalid token: {}", c)
        }
    }

    fn consume(&mut self) {
        if self.current < self.s.len() {
            self.current += 1;
        }
    }

    fn parse(&mut self) -> Expr {
        self.expr().unwrap()
    }

    fn expr(&mut self) -> Option<Expr> {
        let mut node = self.term()?;

        loop {
            let token = self.next_token();
            let e = match token {
                Token::Plus => Expr::Add,
                Token::Mul => Expr::Mul,
                _ => break
            };

            self.consume();
            let right = if token == Token::Mul && self.add_prec == AddPrecedence::Before {
                self.expr()?
            } else {
                self.term()?
            };
            node = e(Box::new(node), Box::new(right));
        }

        Some(node)
    }

    fn term(&mut self) -> Option<Expr> {
        // term = num | par
        self.num().or(self.par())
    }

    fn num(&mut self) -> Option<Expr> {
        // num = (0..9)*
        if !matches!(self.next_token(), Token::Digit(_)) {
            return None;
        }

        let mut s = String::new();

        while let Token::Digit(c) = self.next_token() {
            self.consume();
            s.push(c);
        }

        let n = s.parse().unwrap();
        Some(Expr::Number(n))
    }

    fn par(&mut self) -> Option<Expr> {
        if !matches!(self.next_token(), Token::LeftPar) {
            return None;
        }

        self.consume();
        let e = self.expr()?;
        if self.next_token() != Token::RightPar {
            panic!("Invalid token, expected right parenthesis")
        }
        self.consume();
        Some(e)
    }
}

impl Expr {
    fn from_str(s: &str, add_prec: AddPrecedence) -> Expr {
        ExprParser::new(s, add_prec).parse()
    }

    fn evaluate(&self) -> isize {
        fn eval(expr: &Expr) -> isize {
            match expr {
                Expr::Number(n) => *n,
                Expr::Add(left, right) => eval(left) + eval(right),
                Expr::Mul(left, right) => eval(left) * eval(right),
                Expr::Par(expr) => eval(expr),
                _ => panic!("Can't evaluate expression")
            }
        }
        eval(self)
    }
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| Expr::from_str(line, AddPrecedence::Equal).evaluate())
        .sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| Expr::from_str(line, AddPrecedence::Before).evaluate())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        let s = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(Expr::from_str(s, AddPrecedence::Equal).evaluate(), 71);
        assert_eq!(Expr::from_str(s, AddPrecedence::Before).evaluate(), 231);

        let s = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(Expr::from_str(s, AddPrecedence::Equal).evaluate(), 51);
        assert_eq!(Expr::from_str(s, AddPrecedence::Before).evaluate(), 51);

        let s = "2 * 3 + (4 * 5)";
        assert_eq!(Expr::from_str(s, AddPrecedence::Equal).evaluate(), 26);
        assert_eq!(Expr::from_str(s, AddPrecedence::Before).evaluate(), 46);

        let s = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(Expr::from_str(s, AddPrecedence::Equal).evaluate(), 437);
        assert_eq!(Expr::from_str(s, AddPrecedence::Before).evaluate(), 1445);

        let s = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(Expr::from_str(s, AddPrecedence::Equal).evaluate(), 12240);
        assert_eq!(Expr::from_str(s, AddPrecedence::Before).evaluate(), 669060);

        let s = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(Expr::from_str(s, AddPrecedence::Equal).evaluate(), 13632);
        assert_eq!(Expr::from_str(s, AddPrecedence::Before).evaluate(), 23340);
    }
}

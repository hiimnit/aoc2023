use std::{collections::HashMap, env, fs, iter::Peekable, str::Chars};

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut blocks = input.split("\n\n");

    let workflows_block = blocks
        .next()
        .expect("Expected workflows as the first block");

    let workflows = Workflows::new(workflows_block);

    let parts_block = blocks.next().expect("Expected parts as the second block");

    let mut part_1_result = 0;

    for line in parts_block.lines() {
        let part = line.into();
        if workflows.evaluate_part(&part) {
            part_1_result += part.sum();
        }
    }

    println!("Part 1 result {part_1_result}");
}

#[derive(Debug)]
struct Part {
    x_rating: usize,
    m_rating: usize,
    a_rating: usize,
    s_rating: usize,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut lexer = Lexer::new(value);

        assert!(lexer.next() == Some(Token::Operator('{')));

        assert!(lexer.next() == Some(Token::Identifier("x".into())));
        assert!(lexer.next() == Some(Token::Operator('=')));
        let Some(Token::Number(x_rating)) = lexer.next() else {
            panic!("Expected a valid usize");
        };
        assert!(lexer.next() == Some(Token::Operator(',')));

        assert!(lexer.next() == Some(Token::Identifier("m".into())));
        assert!(lexer.next() == Some(Token::Operator('=')));
        let Some(Token::Number(m_rating)) = lexer.next() else {
            panic!("Expected a valid usize");
        };
        assert!(lexer.next() == Some(Token::Operator(',')));

        assert!(lexer.next() == Some(Token::Identifier("a".into())));
        assert!(lexer.next() == Some(Token::Operator('=')));
        let Some(Token::Number(a_rating)) = lexer.next() else {
            panic!("Expected a valid usize");
        };
        assert!(lexer.next() == Some(Token::Operator(',')));

        assert!(lexer.next() == Some(Token::Identifier("s".into())));
        assert!(lexer.next() == Some(Token::Operator('=')));
        let Some(Token::Number(s_rating)) = lexer.next() else {
            panic!("Expected a valid usize");
        };

        assert!(lexer.next() == Some(Token::Operator('}')));

        Self { x_rating, m_rating, a_rating, s_rating }
    }
}

impl Part {
    fn sum(&self) -> usize {
        self.x_rating + self.m_rating + self.a_rating + self.s_rating
    }
}

#[derive(Debug)]
struct Workflows {
    workflows: HashMap<String, Workflow>,
}

impl Workflows {
    fn new(input: &str) -> Self {
        let mut workflows = HashMap::new();

        for line in input.lines() {
            let mut lexer = Lexer::new(line);

            let Some(Token::Identifier(workflow_name)) = lexer.next() else {
                panic!("Expected an identifier as workflow name");
            };

            assert!(lexer.next() == Some(Token::Operator('{')));

            let rule = parse_rule(&mut lexer);

            assert!(lexer.next() == Some(Token::Operator('}')));
            
            workflows.insert(workflow_name, Workflow { first_rule: rule });
        }

        Self {
            workflows,
        }
    }

    fn evaluate_part(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];

        loop {
            match workflow.evaluate_part(part) {
                Rule::GoToWorkflow(next_workflow) => {
                    workflow = &self.workflows[next_workflow];
                }
                Rule::Comparison(_) => panic!("Unexpected return value - Comparison"),
                Rule::Accepted => return true,
                Rule::Rejected => return false,
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    first_rule: Rule,
}

impl Workflow {
    fn evaluate_part(&self, part: &Part) -> &Rule {
        match &self.first_rule {
            Rule::Comparison(comparison) => comparison.evaluate_part(part),
            rule => rule,
        }
    }
}

#[derive(Debug)]
struct Comparison {
    category: Category,
    condition: Condition,
    threshold: usize,

    when_true: Box<Rule>,
    when_false: Box<Rule>,
}

fn parse_rule(lexer: &mut Lexer) -> Rule {
    let Some(token) = lexer.next() else {
        panic!("Unexpected end of stream");
    };

    let identifier = match token {
        Token::Identifier(identifier) => identifier,
        Token::Operator(_) | Token::Number(_) => {
            panic!("Expected an identifier, got {token:?} instead.")
        }
    };

    match identifier.as_str() {
        "A" => return Rule::Accepted,
        "R" => return Rule::Rejected,
        _ => {}
    }

    let condition = match lexer.peek_next() {
        Some(Token::Operator('<')) | Some(Token::Operator('>')) => {
            let operator = lexer.next().unwrap();

            match operator {
                Token::Operator('<') => Condition::LessThan,
                Token::Operator('>') => Condition::GreaterThan,
                _ => unreachable!(),
            }
        }
        _ => return Rule::GoToWorkflow(identifier),
    };

    let Some(Token::Number(threshold)) = lexer.next() else { 
        panic!("Expected a number after a condition");
    };

    assert!(lexer.next() == Some(Token::Operator(':')));
    let when_true = Box::new(parse_rule(lexer));
    assert!(lexer.next() == Some(Token::Operator(',')));
    let when_false = Box::new(parse_rule(lexer));

    Rule::Comparison(Comparison {
        category: identifier.into(),
        condition,
        threshold,
        when_true,
        when_false,
    })
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    peeked_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            peeked_token: None,
        }
    }

    fn peek_next(&mut self) -> &Option<Token> {
        if self.peeked_token.is_some() {
            return &self.peeked_token;
        }

        self.peeked_token = self.next();

        &self.peeked_token
    }

    fn next(&mut self) -> Option<Token> {
        if self.peeked_token.is_some() {
            return self.peeked_token.take();
        }

        let Some(next_char) = self.chars.next() else {
            return None;
        };

        if is_operator(&next_char) {
            return Some(Token::Operator(next_char));
        }

        if next_char >= '0' && next_char <= '9' {
            return Some(Token::Number(self.next_number(next_char)));
        }

        let mut identifier = next_char.to_string();

        loop {
            match self.chars.peek() {
                Some(c) => {
                    if is_operator(c) {
                        break;
                    }
                }
                None => break,
            }

            identifier.push(self.chars.next().unwrap());
        }

        Some(Token::Identifier(identifier))
    }

    fn next_number(&mut self, first_digit: char) -> usize {
        let mut number = char_to_usize(first_digit);

        loop {
            match self.chars.peek() {
                Some(c) => {
                    if *c < '0' || *c > '9' {
                        break;
                    }
                }
                None => break,
            }

            number = number * 10 + char_to_usize(self.chars.next().unwrap());
        }

        number
    }
}

fn char_to_usize(c: char) -> usize {
    (c as u8 - '0' as u8) as usize
}

fn is_operator(c: &char) -> bool {
    match c {
        '{' | '}' | '<' | '>' | ':' | ',' | '=' => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    Operator(char),
    Number(usize),
}

impl Comparison {
    fn evaluate_part(&self, part: &Part) -> &Rule {
        let Comparison {
            category,
            condition,
            threshold,
            when_true,
            when_false,
        } = &self;

        let rating = match category {
            Category::X => part.x_rating,
            Category::M => part.m_rating,
            Category::A => part.a_rating,
            Category::S => part.s_rating,
        };

        let result = match condition {
            Condition::LessThan => rating < *threshold,
            Condition::GreaterThan => rating > *threshold,
        };

        let result = if result { when_true.as_ref() } else { when_false.as_ref() };

        match result {
            Rule::Comparison(rule) => rule.evaluate_part(part),
            result => result,
        }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<String> for Category {
    fn from(value: String) -> Self {
        match value.as_str() {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Invalid category {value}"),
        }
    }
}

#[derive(Debug)]
enum Rule {
    GoToWorkflow(String),
    Comparison(Comparison),
    Accepted,
    Rejected,
}

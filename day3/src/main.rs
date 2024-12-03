use regex::Regex;

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Cannot read file!");
    let res = compute(&file);
    assert_eq!(59097164, res);

    let res2 = solution_without_regex(&file.as_str());
    assert_eq!(59097164, res2);
}

///
/// Part 1 and 2
/// Matches all the 'mul(?,?)'s, 'do()'s, and 'don't()'s, extracts the 
/// values and adds them if the previous conditional capture was a 'do()'
/// Finally returns the sum of the uncorrupted multiplications 
/// 
fn compute(file: &String) -> i64 {
    let pattern = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").expect("Invalid regex");
    let mut add = true;

    let mut res: i64 = 0;
    for capture in pattern.find_iter(file) {
        let cap = capture.as_str();
        match cap {
            "do()" => add = true, 
            "don't()" => add = false,
            _ if add => {
                let values: Vec<i64> = cap[4..cap.len() - 1].split(',') // remove leading 'mul(' and last ')' and split by comma
                    .map(|e| e.parse().expect("Not an int!"))
                    .collect();
                assert_eq!(2, values.len());
                res += values[0] * values[1];
            },
            _ => (),
        }
    }
    res
}



///
/// Part 1 and 2 solution without regex!!
/// 
struct Lexer<'a> {
    content: &'a str,
    ptr: usize, 
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content, ptr: 0 }
    }

    pub fn current_char(&self) -> Option<char> {
        if self.ptr + 1 <= self.content.len() {
            self.content.chars().nth(self.ptr)
        } else {
            None
        }
    }
    pub fn advance(&mut self, adv: usize) {
        self.ptr += adv
    }

    pub fn check_matches_and_advance(&mut self, check: &str) -> bool {
        if self.ptr + check.len() <= self.content.len() {
            let res = check.eq(&self.content[self.ptr..self.ptr + check.len()]);
            if res {
                self.advance(check.len());
            }
            res
        } else {
            false
        }
    }
}

fn solution_without_regex(content: &str) -> i64 {
    let mut lexer = Lexer::new(content); 

    let mut ret: i64 = 0;
    let mut add: bool = true;
    while let Some(ch) = lexer.current_char() {
        match ch {
            'd' => parse_do_dont(&mut lexer, &mut add),
            'm' => {
                let (left, right) = parse_mul(&mut lexer);
                if add {
                    ret += left * right
                }
            },
            _ => lexer.advance(1),
        } 
    }
    ret
}

fn parse_do_dont(lexer: &mut Lexer, add: &mut bool) {
    if lexer.check_matches_and_advance("do()") {
        *add = true;
    } else if lexer.check_matches_and_advance("don't()") {
        *add = false;
    } else {
        lexer.advance(1);
    }
}

fn parse_mul(lexer: &mut Lexer) -> (i64, i64) {
    if !lexer.check_matches_and_advance("mul(") {
        lexer.advance(1);
        return (0, 0)
    }
    
    let left = parse_number(lexer);
    if !lexer.check_matches_and_advance(",") {
        return (0, 0)
    }
    
    let right = parse_number(lexer);
    if !lexer.check_matches_and_advance(")") {
        return (0, 0)
    }

    (left, right)
}

fn parse_number(lexer: &mut Lexer) -> i64 {
    let mut number_str = String::new();

    while let Some(ch) = lexer.current_char() {
        if ch.is_ascii_digit() {
            number_str.push(ch);
            lexer.advance(1)
        } else {
            break;
        }
    }

    match number_str.parse::<i64>() {
        Ok(val) => val,
        Err(_) => 0,
    }
}


#[cfg(test)]
#[test]
fn test1() {
    let mut l = Lexer::new("357");
    assert_eq!(parse_number(&mut l), 357);
}

#[test]
fn test_mul() {
    let res = solution_without_regex("mul(100,10)");
    assert_eq!(res, 1000)
}





use std::env;
use std::fs::File;
use std::io::prelude::*;

mod token {
    pub const gt: char = '>';
    pub const lt: char = '<';
    pub const plus: char = '+';
    pub const minus: char = '-';
    pub const period: char = '.';
    pub const comma: char = ',';
    pub const lbracket: char = '[';
    pub const rbracket: char = ']';
    pub const eof: char = 0 as char;
}

struct Brainfuck {
    buf: [u8; 30000],
    buf_ptr: usize,
    source: String,
    source_ptr: usize,
}

impl Brainfuck {
    pub fn new(source: String) -> Brainfuck {
        Brainfuck {
            buf: [0; 30000],
            buf_ptr: 0,
            source: source,
            source_ptr: 0,
        }
    }

    fn read_char(&mut self) -> char {
        if self.source_ptr >= self.source.len() {
            return 0 as char;
        } else {
            let cs: Vec<char> = self.source.clone().chars().collect();
            let c: char = cs[self.source_ptr];
            self.source_ptr += 1;
            c
        }
    }

    fn get_char(&self, index: usize) -> char {
        let cs: Vec<char> = self.source.clone().chars().collect();
        let c: char = cs[index];
        c
    }

    fn eval(&mut self) {
        while self.eval_char() {}
    }

    fn eval_char(&mut self) -> bool {
        let c = self.read_char();
        let mut result = true;
        match c {
            token::gt => {
                self.eval_gt();
            }
            token::lt => {
                self.eval_lt();
            }
            token::plus => {
                self.eval_plus();
            }
            token::minus => {
                self.eval_minus();
            }
            token::period => {
                self.eval_period();
            }
            token::comma => {
                self.eval_comma();
            }
            token::lbracket => {
                self.eval_lbracket();
            }
            token::rbracket => {
                self.eval_rbracket();
            }
            token::eof => {
                result = false;
            }
            _ => {
                result = true;
            }
        }
        result
    }

    fn eval_gt(&mut self) {
        self.buf_ptr += 1;
        if self.buf_ptr > 30000 {
            panic!("The pointer has gone out of memory");
        }
    }

    fn eval_lt(&mut self) {
        self.buf_ptr -= 1;
        if self.buf_ptr < 0 {
            panic!("The pointer has gone out of memory");
        }
    }

    fn eval_plus(&mut self) {
        self.buf[self.buf_ptr] += 1;
    }

    fn eval_minus(&mut self) {
        self.buf[self.buf_ptr] -= 1;
    }

    fn eval_period(&self) {
        print!("{}", self.buf[self.buf_ptr] as char);
    }

    fn eval_comma(&self) {
        panic!("Comma evaluation is not implemented.");
    }

    fn eval_lbracket(&mut self) {
        if self.buf[self.buf_ptr] == 0 {
            while self.get_char(self.source_ptr + 1) != ']' {
                self.source_ptr += 1
            }
        }
    }

    fn eval_rbracket(&mut self) {
        if self.buf[self.buf_ptr] != 0 {
            while self.get_char(self.source_ptr - 1) != '[' {
                self.source_ptr -= 1
            }
        }
    }
}

fn main() {
    let mut filename = String::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        filename = args[1].clone();
    } else {
        println!("please input source file.");
        return;
    }

    let mut f = File::open(filename).expect("file not found");

    let mut source = String::new();
    f.read_to_string(&mut source)
        .expect("something went wrong reading the file");

    let mut bf = Brainfuck::new(source);
    bf.eval();
}

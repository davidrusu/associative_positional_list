
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering;


enum Item {
    Integer(usize),
    List(Box<Vec<Item>>),
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Item::Integer(x) => match other {
                Item::Integer(y) => {
                    return x.cmp(y);
                },
                Item::List(_) => {
                    let xl = Item::List(Box::new(vec![Item::Integer(*x)]));
                    return xl.cmp(other);
                },
            },
            Item::List(x) => match other {
                Item::Integer(y) => {
                    let yl = Item::List(Box::new(vec![Item::Integer(*y)]));
                    return self.cmp(&yl);
                },
                Item::List(y) => {
                    for i in 0 .. usize::min(x.len(), y.len()) {
                        let xi = x.get(i).unwrap();
                        let yi = y.get(i).unwrap();
                        let c = xi.cmp(yi);
                        if c != Ordering::Equal {
                            return c;
                        }
                    }
                    return x.len().cmp(&y.len());
                },
            },
        };
    }
}

struct Pair {
    left: Item,
    right: Item,
}

type Problem = Vec<Pair>;

fn load(filename: &str) -> Problem {
    let file = File::open(filename).unwrap();
    let mut problem: Problem = Vec::new();
    let mut stack: Vec<Item> = Vec::new();
    let mut line_number: usize = 0;

    for line in io::BufReader::new(file).lines() {
        line_number += 1;
        if let Ok(line_string) = line {
            let mut integer_bytes: usize = 0;
            let mut integer_value: usize = 0;
            assert!(stack.is_empty());
            stack.push(Item::List(Box::new(Vec::new())));
            for ch in line_string.bytes() {
                match ch {
                    b']' | b',' | b'\n' => {
                        if integer_bytes != 0 {
                            match stack.last_mut().unwrap() {
                                Item::Integer(_) => {
                                    panic!();
                                },
                                Item::List(x) => {
                                    x.push(Item::Integer(integer_value));
                                },
                            }
                        }
                        integer_bytes = 0;
                        integer_value = 0;
                    },
                    b'[' => {
                        assert!(integer_bytes == 0);
                    },
                    _ => {
                        let digit = (ch - b'0') as usize;
                        assert!(digit < 10);
                        integer_bytes += 1;
                        integer_value *= 10;
                        integer_value += digit;
                    },
                }
                match ch {
                    b'[' => {
                        stack.push(Item::List(Box::new(Vec::new())));
                    },
                    b']' => {
                        let child = stack.pop().unwrap();
                        match stack.last_mut().unwrap() {
                            Item::Integer(_) => {
                                panic!();
                            },
                            Item::List(x) => {
                                x.push(child);
                            },
                        }
                    },
                    _ => {},
                }
            }
            assert_eq!(stack.len(), 1);
            match line_number % 3 {
                1 => {
                    problem.push(Pair {
                        left: stack.pop().unwrap(),
                        right: Item::List(Box::new(Vec::new())),
                    });
                },
                2 => {
                    problem.last_mut().unwrap().right = stack.pop().unwrap();
                },
                _ => {
                    stack.pop();
                },
            }
            assert!(stack.is_empty());
        }
    }
    return problem;
}

fn print_list(item: &Item) {
    match item {
        Item::Integer(_) => {
            panic!();
        },
        Item::List(x) => {
            let mut first = true;
            for y in x.iter() {
                if !first {
                    print!(",");
                }
                print_item(y);
                first = false;
            }
        },
    }
}

fn print_item(item: &Item) {
    match item {
        Item::Integer(x) => {
            print!("{}", x);
        },
        Item::List(_) => {
            print!("[");
            print_list(&item);
            print!("]");
        }
    }
}

fn print_problem(problem: &Problem) {
    for pair in problem {
        print_list(&pair.left);
        println!();
        print_list(&pair.right);
        println!();
        println!();
    }
}


fn main() {
    let p = load("input");
    print_problem(&p);
}



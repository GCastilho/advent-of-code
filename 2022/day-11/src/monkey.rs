use std::collections::VecDeque;
use meval::Expr;

#[derive(Debug)]
pub struct Monkey {
    pub name: String,
    pub items: VecDeque<i32>,
    pub dest: (usize, usize),
    operation_raw_expr: String,
    test_val: i32,
}

impl Monkey {
    pub fn new(monkey: &str) -> Self {
        let mut monkey = monkey.lines();

        let name = monkey.next().unwrap()
            .split(":")
            .next()
            .unwrap()
            .to_owned();

        let starting_items = monkey.next().unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|v| v.trim().parse::<i32>().unwrap())
            .collect::<VecDeque<_>>();

        let operation = monkey.next().unwrap()
            .split("=")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .to_string();

        let test = monkey.next().unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut parse_case_line = || {
            monkey.next().unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        };

        let case_true = parse_case_line();
        let case_false = parse_case_line();

        Monkey {
            name,
            items: starting_items,
            operation_raw_expr: operation,
            test_val: test,
            dest: (case_true, case_false),
        }
    }

    pub fn test(&self, input: i32) -> bool {
        input % self.test_val == 0
    }

    pub fn inspect(&self, input: i32) -> i32 {
        let expr = self.operation_raw_expr.parse::<Expr>()
            .unwrap()
            .bind("old")
            .unwrap();
        expr(input as f64) as i32
    }

    pub fn get_item(&mut self) -> i32 {
        self.items.pop_front().unwrap()
    }
}

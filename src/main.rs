mod balancer;
mod balancer_error;
mod element;
mod entity;
mod equation;
mod group;
mod item;
mod matrix;
mod parser;
mod parser_error;
mod regular_expression;

use std::env;
use crate::balancer::Balancer;

fn main() {
    let equation = match env::args().nth(1) {
        Some(x) => x,
        None => panic!("Нет уравнения."),
    };
    let mut balancer = match Balancer::new(&equation) {
        Ok(x) => x,
        Err(e) => panic!(e.get_description()),
    };
    let balanced_equation = match balancer.balance_equation() {
        Ok(x) => x,
        Err(e) => panic!(e.get_description()),
    };
    println!("{}", balanced_equation);
}

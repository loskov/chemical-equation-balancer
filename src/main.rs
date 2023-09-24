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
    let equation = env::args().nth(1).expect("No equation.");
    let mut balancer = Balancer::new(&equation)
        .unwrap_or_else(|parser_error| panic!("{}", parser_error.get_description()));
    let balanced_equation = balancer
        .balance_equation()
        .unwrap_or_else(|balancer_error| panic!("{}", balancer_error.get_description()));

    println!("{balanced_equation}");
}

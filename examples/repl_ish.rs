use std::error::Error;
use prettytable;
use prettytable::{Table, Row, Cell, Attr, color};
use std::boxed::Box;
use std::io::{self, Read, Write};
use std::fmt::Debug;
use meval;
use crate::ReplState::{ENTER_EQUATION, ENTER_POINTS, SOLVE_POINTS, ENTER_INITIAL_CONDITION, GRAPH_EQUATION, GET_INTENT};
use std::collections::HashMap;
use num;
use ctrlc;

use diffy_queue::{runge_kutta, euler};
use diffy_queue::solver::Solver;
use std::fs::OpenOptions;
use std::num::ParseFloatError;

enum ReplState {
    ENTER_EQUATION,
    ENTER_POINTS,
    ENTER_INITIAL_CONDITION,
    SOLVE_POINTS,
    GRAPH_EQUATION,
    GET_INTENT
}

fn print_chars(to_print: &mut String) {
    for c in to_print.chars() {
        print!("'{}', ", c);
    }
    println!();
}

fn input_as<T>(buffer: &mut String, prompt: &str) -> T
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: Debug {
    loop {
        buffer.clear();
        print!("{}", prompt);
        io::stdout().flush();
        io::stdin().read_line(buffer);
        print_chars(&mut buffer.trim().to_string());
        match buffer.trim().parse::<T>() {
            Ok(t) => return t,
            Err(e) => {
                println!("Unable to parse input as {}", std::any::type_name::<T>());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        println!("\nReceived interrupt, exiting...");
        std::process::exit(1);
    });

    let mut buffer = String::new();
    let mut should_cont = true;
    let mut state = ReplState::ENTER_EQUATION;

    let mut equation: Option<meval::Expr> = None;
    let mut initial = (0f32, 0f32);
    let mut points: Vec<f32> = vec![];

    while should_cont {
        buffer.clear();
        match state {
            ENTER_EQUATION => {
                println!("Please enter equation, as a function of x and t.");
                print!("dx/dt = ");
                io::stdout().flush();
                io::stdin().read_line(&mut buffer);
                match buffer.trim().parse::<meval::Expr>() {
                    Ok(t) => { equation = Some(t); state = ENTER_INITIAL_CONDITION; }
                    Err(e) => println!("Error: unable to parse expression: {}", e)
                }
            },
            ENTER_INITIAL_CONDITION=> {
                println!("Please enter initial condition");
                initial.0 = input_as(&mut buffer, "t_0 = ");
                initial.1 = input_as(&mut buffer, "x_0 = ");
                state = ENTER_POINTS;
            },
            ENTER_POINTS => {
                println!("Please enter a series of t-values for which you'd like to generate a solution");
                println!("When you are finished inputting values, please enter any non-numeric value");
                let mut i = 1;
                loop {
                    print!("t_{} = ", i);
                    io::stdout().flush();
                    i += 1;
                    buffer.clear();
                    io::stdin().read_line(&mut buffer);
                    println!("{}", buffer);
                    println!("len: {}", buffer.len());
                    match buffer.trim().parse::<f32>() {
                        Ok(num) => points.push(num),
                        Err(_) => { state = SOLVE_POINTS; break }
                    }
                }
            },
            SOLVE_POINTS => {
                let h = input_as(&mut buffer, "Please enter step size\nh = ");

                let e;
                if let Some(eqn) = equation.clone() {
                    e = eqn.clone().bind2("t", "x").unwrap();
                } else {
                    println!("Error: Equation is None (you shouldn't be seeing this)");
                    state = ENTER_EQUATION;
                    continue;
                }
                let f = move |t: f32, x: f32| e(t as f64, x as f64) as f32;

                let mut euler = euler::create_euler_solver(&f, initial.0, initial.1, h);
                let mut rk4 = runge_kutta::create_rk4_solver(&f, initial.0, initial.1, h);
                let mut table = Table::new();
                table.add_row(Row::new(vec![
                    Cell::new("t").with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::GREEN)),
                    Cell::new("Euler Sol'n").with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::RED)),
                    Cell::new("Runge-Kutta (4th order) Sol'n").with_style(Attr::Blink)
                        .with_style(Attr::ForegroundColor(color::BLUE))
                ]));
                for point in &points {
        `            table.add_row(Row::new(vec![
                        Cell::new(point.to_string().as_str())
                            .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
                        Cell::new(euler.solve_at_point(*point).unwrap().to_string().as_str())
                            .with_style(Attr::ForegroundColor(color::BRIGHT_RED)),
                        Cell::new(rk4.solve_at_point(*point).unwrap().to_string().as_str())
                            .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE))
                    ]));
                }
                table.printstd();
                state = GRAPH_EQUATION;
            },
            GRAPH_EQUATION => {
                println!("Graphing from repl not yet implemented");
                state = GET_INTENT;
            }
            GET_INTENT => {
                print!("Would you like to continue? [y/n]: ");
                io::stdin().read_line(&mut buffer);
                buffer = buffer.to_lowercase();
                if buffer == "y" {
                    state = ENTER_EQUATION;
                } else {
                    println!("Exiting...");
                    should_cont = false;
                }
            }
        }
    }

    return Ok(());
}
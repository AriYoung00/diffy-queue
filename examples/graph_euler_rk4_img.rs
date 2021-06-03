use plotters::prelude::*;
use std::f64::consts::E;

use diffy_queue::runge_kutta::{RK4Solver, DOPRISolver};
use diffy_queue::solver::Solver;
use diffy_queue::euler::EulerSolver;

fn graph_series_in_bounds(x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
    let root = BitMapBackend::new("images/test_chart.png", (900, 600))
        .into_drawing_area();

    root.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root)
        .caption("dx/dt = t*x, h=0.01", ("Arial", 30))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_min as f64..x_max as f64, y_min as f64..y_max as f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let f_t_x = |t, x| t*x;
    let mut solver = EulerSolver::new(f_t_x, 0.0, 1.0, 0.01);
    let mut rk4_solver = RK4Solver::new(&f_t_x, 0.0, 1.0, 0.01);
    let mut dopri_solver = DOPRISolver::new(&f_t_x, 0.0, 1.0, 0.01, 0.001, 1.0);
    let actual_soln = |t: f64| E.powf(0.5 * t.powf(2.0));

    // ctx.draw_series(
    //     LineSeries::new((0..=x_max*10)
    //                         .map(|x| x as f64 / 10.0)
    //                         .map(|x| (x, solver.solve_at_point(x).unwrap())), &RED)
    // ).unwrap()
    //     .label("Euler")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    //
    // ctx.draw_series(
    //     LineSeries::new((x_min*10..=x_max*10)
    //                         .map(|x| x as f64 / 10.0)
    //                         .map(|x| (x, rk4_solver.solve_at_point(x).unwrap())), &BLUE)
    // ).unwrap()
    //     .label("Runge-Kutta")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.draw_series(
        LineSeries::new((0..=x_max*10)
                            .map(|x| x as f64 / 10.0)
                            .map(|x| (x, dopri_solver.solve_at_point(x).unwrap())), &BLUE)
    ).unwrap()
        .label("Dormand-Prince")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.draw_series(
        LineSeries::new((x_min*10..=x_max*10)
                            .map(|x| x as f64 / 10.0)
                            .map(|x| (x, actual_soln(x))), &GREEN)
    ).unwrap()
        .label("Actual solution")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    ctx.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();
}

fn main() {
    graph_series_in_bounds(0, 3, 0, 80);
}


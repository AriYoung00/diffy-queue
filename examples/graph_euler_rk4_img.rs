use plotters::prelude::*;
use std::f32::consts::E;

use diffy_queue::euler;
use diffy_queue::runge_kutta as rk;
use diffy_queue::solver::Solver;

fn graph_series_in_bounds(x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
    let root = BitMapBackend::new("images/test_chart.png", (900, 600))
        .into_drawing_area();

    root.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root)
        .caption("Test Figure", ("Arial", 30))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_min as f32..x_max as f32, y_min as f32..y_max as f32)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let f_t_x = |t, x| t*x;
    let mut solver = euler::create_euler_solver(f_t_x, 0.0, 1.0, 0.01);
    let mut rk4_solver = rk::create_rk4_solver(f_t_x, 0.0, 1.0, 0.01);
    let actual_soln = |t: f32| E.powf(0.5 * t.powf(2.0));

    println!("computed(1.0) = {}", solver.solve_at_point(1.0).unwrap());

    ctx.draw_series(
        LineSeries::new((x_min*10..x_max*10)
                            .map(|x| x as f32 / 10.0)
                            .map(|x| (x, solver.solve_at_point(x).unwrap())), &RED)
    ).unwrap()
        .label("Euler")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(
        LineSeries::new((x_min*10..=x_max*10)
                            .map(|x| x as f32 / 10.0)
                            .map(|x| (x, rk4_solver.solve_at_point(x).unwrap())), &BLUE)
    ).unwrap()
        .label("Runge-Kutta")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.draw_series(
        LineSeries::new((x_min*10..=x_max*10)
                            .map(|x| x as f32 / 10.0)
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
    graph_series_in_bounds(2, 3, 0, 80);
}


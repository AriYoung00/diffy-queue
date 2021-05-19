use plotters::prelude::*;
use std::f32::consts::E;

use diffy_queue::euler;
use diffy_queue::solver::Solver;

fn main() {
    let root = BitMapBackend::new("images/test_chart.png", (900, 600))
        .into_drawing_area();

    root.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root)
        .caption("Test Figure", ("Arial", 30))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0f32..4f32, 0f32..350f32)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let f_t_x = |t, x| t*x;
    let mut solver = euler::create_euler_solver(f_t_x, 0.0, 1.0, 0.1);
    let actual_soln = |t: f32| E.powf(0.5 * t.powf(2.0));

    println!("computed(1.0) = {}", solver.solve_at_point(1.0).unwrap());

    ctx.draw_series(
        LineSeries::new((0..=40)
            .map(|x| x as f32 / 10.0)
            .map(|x| (x, solver.solve_at_point(x).unwrap())), &RED)
    ).unwrap();
    ctx.draw_series(
        LineSeries::new((0..=40)
                            .map(|x| x as f32 / 10.0)
                            .map(|x| (x, actual_soln(x))), &GREEN)
    ).unwrap();
}


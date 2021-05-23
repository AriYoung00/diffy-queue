use crate::solver::Solver;
use std::cmp::Ordering::Equal;

pub struct EulerSolver<'a> {
    t_curr: f32,
    x_curr: f32,
    step_size: f32,
    solved_pts: Vec<(f32, f32)>,
    f: &'a dyn Fn(f32, f32) -> f32,
}

impl Solver for EulerSolver<'_> {
    fn solve_at_point(&mut self, t: f32) -> Option<f32> {
        println!("\nSolving at t={}, max={}", t, self.t_curr);
        if t > self.t_curr {
            println!("Greater than max, iterating");
            for point in self {
                if point.0 >= t {
                    return Some(point.1);
                }
            }
            None
        } else {
            println!("Less than max, searching solution list");
            match self.solved_pts.binary_search_by(|a| a.0.partial_cmp(&t)
                                                                    .unwrap_or(Equal)) {
                Ok(idx) => Some(self.solved_pts[idx].0),
                Err(idx) => Some(self.solved_pts[idx].0)
            }
        }
    }

    fn _next(&mut self) -> Option<(f32, f32)> {
        let t_prev = self.t_curr;
        let x_prev = self.x_curr;

        self.t_curr += self.step_size;
        self.x_curr += self.step_size * (self.f)(t_prev, x_prev);
        println!("Step, t_curr: {}, x_curr: {}", self.t_curr, self.x_curr);

        self.solved_pts.push((self.t_curr, self.x_curr));
        Some((self.t_curr, self.x_curr))
    }
}

impl Iterator for EulerSolver<'_> {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<(f32, f32)> { self._next() }
}

pub fn create_euler_solver (f: &dyn Fn(f32, f32) -> f32, t_0: f32, x_0: f32, step_size: f32) ->
                                                                        EulerSolver {
    EulerSolver {
        t_curr: t_0,
        x_curr: x_0,
        step_size,
        solved_pts: vec![(t_0, x_0)],
        f
    }
}
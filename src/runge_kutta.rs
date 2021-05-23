use crate::solver::Solver;
use std::cmp::Ordering::Equal;

pub struct RK4Solver<'a> {
    t_curr: f32,
    x_curr: f32,
    step_size: f32,
    solved_pts: Vec<(f32, f32)>,
    f: &'a dyn Fn(f32, f32) -> f32,
}

impl Solver for RK4Solver<'_> {
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
        let y_n = self.x_curr;
        let t_n = self.t_curr;
        let h = self.step_size;
        let f = self.f;

        let k_1 = f(y_n, t_n);
        let k_2 = f(t_n + h/2.0, y_n + h*k_1.clone()/2.0);
        let k_3 = f(t_n + h/2.0, y_n + h*k_2.clone()/2.0);
        let k_4 = f(t_n + h, y_n + h*k_3.clone());

        self.x_curr = y_n + 1.0/6.0*h*(k_1 + 2.0*k_2 + 2.0*k_3 + k_4);
        self.t_curr = t_n + h;

        self.solved_pts.push((self.t_curr, self.x_curr));
        Some((self.t_curr, self.x_curr))
    }
}

impl Iterator for RK4Solver<'_> {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        self._next()
    }
}

pub fn create_rk4_solver(f: &dyn Fn(f32, f32) -> f32, t_0: f32, x_0: f32, step_size: f32) -> RK4Solver {
    RK4Solver {
        t_curr: t_0,
        x_curr: x_0,
        step_size,
        solved_pts: vec![(t_0, x_0)],
        f
    }
}

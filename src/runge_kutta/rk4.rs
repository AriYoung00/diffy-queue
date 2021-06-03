use crate::solver::Solver;
use std::cmp::Ordering::Equal;

pub struct RK4Solver<'a> {
    t_curr: f64,
    x_curr: f64,
    step_size: f64,
    pub solved_pts: Vec<(f64, f64)>,
    f: Box<dyn Fn(f64, f64) -> f64 + 'a>,
}

impl<'a> RK4Solver<'a> {
    pub fn new(f: impl Fn(f64, f64) -> f64 + 'a, t_0: f64, x_0: f64, step_size: f64) -> Self {
        RK4Solver {
            t_curr: t_0,
            x_curr: x_0,
            step_size,
            solved_pts: vec![(t_0, x_0)],
            f: Box::new(f)
        }
    }
}

impl<'a> Solver<'a> for RK4Solver<'a> {
    fn solve_at_point(&mut self, t: f64) -> Option<f64> {
        if t > self.t_curr {
            for point in self {
                if point.0 >= t {
                    return Some(point.1);
                }
            }
            None
        } else {
            match self.solved_pts.binary_search_by(|a| a.0.partial_cmp(&t)
                .unwrap_or(Equal)) {
                Ok(idx) => Some(self.solved_pts[idx].1),
                Err(idx) => Some(self.solved_pts[idx].1)
            }
        }
    }

    fn set_step_size(&mut self, h: f64) {
        self.step_size = h;
        let initial_pt = self.solved_pts[0];
        self.solved_pts.clear();
        self.solved_pts.push(initial_pt);
        (self.t_curr, self.x_curr) = initial_pt;
    }

    fn set_fn(&mut self, f: impl Fn(f64, f64) -> f64 + 'a) {
        let initial_pt = self.solved_pts[0];
        self.solved_pts.clear();
        self.solved_pts.push(initial_pt);
        (self.t_curr, self.x_curr) = initial_pt;
        self.f = Box::new(f);
    }

    fn set_initial_pt(&mut self, pt: (f64, f64)) {
        self.solved_pts.clear();
        (self.t_curr, self.x_curr) = pt;
        self.solved_pts.push(pt);
    }

    fn _next(&mut self) -> Option<(f64, f64)> {
        let y_n = self.x_curr;
        let t_n = self.t_curr;
        let h = self.step_size;
        let f = &self.f;

        let k_1 = f(y_n, t_n);
        let k_2 = f(t_n + h/2.0, y_n + h*k_1.clone()/2.0);
        let k_3 = f(t_n + h/2.0, y_n + h*k_2.clone()/2.0);
        let k_4 = f(t_n + h, y_n + h*k_3.clone());

        self.x_curr = y_n + 1.0/6.0*h*(k_1 + 2.0*k_2 + 2.0*k_3 + k_4);
        self.t_curr = t_n + h;
        self.t_curr = (self.t_curr * 100000000.0).round() / 100000000.0;

        self.solved_pts.push((self.t_curr, self.x_curr));
        Some((self.t_curr, self.x_curr))
    }
}

impl Iterator for RK4Solver<'_> {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        self._next()
    }
}


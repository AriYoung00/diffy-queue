use crate::solver::Solver;
use std::cmp::Ordering::Equal;

const A: [[f64; 6]; 7] = [
    [0.0; 6],
    [1.0/5.0, 0., 0., 0., 0., 0.],
    [3./40., 9./40., 0., 0., 0., 0.],
    [44./45., -56./15., 32./9., 0., 0., 0.],
    [19372./6561., -25360./2187., 64448./6561., -212./729., 0., 0.],
    [9017./3168., -355./33., 46732./5247., 49./176., -5103./18656., 0.],
    [35./384., 0., 500./1113. ,125./192., -2187./6784., 11./84.],
];
const C: [f64; 7] = [0., 1.0/5.0, 3.0/10.0, 4.0/5.0, 8.0/9.0, 1.0, 1.0];
const B1: [f64; 7] = [35./384., 0., 500./1113., 125./192., -2187./6784., 11./84., 0.];
const B2: [f64; 7] = [5179./57600., 0., 7571./16695., 393./640., -92097./339200., 187./2100., 1./40.];

pub struct DOPRISolver<'a> {
    t_curr: f64,
    x_curr: f64,
    h: f64,
    h_max: f64,
    h_min: f64,
    pub solved_pts: Vec<(f64, f64)>,
    f: Box<dyn Fn(f64, f64) -> f64 + 'a>
}

impl<'a> DOPRISolver<'a> {
    pub fn new(f: impl Fn(f64, f64) -> f64 + 'a, t_0: f64, x_0: f64, step_size: f64, h_min: f64, h_max: f64) -> Self {
        DOPRISolver {
            t_curr: t_0,
            x_curr: x_0,
            h: step_size,
            h_min,
            h_max,
            solved_pts: vec![(t_0, x_0)],
            f: Box::new(f)
        }
    }

    pub fn set_step_bounds(&mut self, bounds: (f64, f64)) {
        self._reset();
        (self.h_min, self.h_max) = bounds;
    }

    fn _reset(&mut self) {
        let initial_pt = self.solved_pts[0];
        self.solved_pts.clear();
        (self.t_curr, self.x_curr) = initial_pt;
        self.solved_pts.push(initial_pt);
    }
}

impl Iterator for DOPRISolver<'_> {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> { self._next() }
}

impl<'a> Solver<'a> for DOPRISolver<'a> {
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
        self._reset();
        self.h = h;
    }

    fn set_fn(&mut self, f: impl Fn(f64, f64) -> f64 + 'a) {
        self._reset();
        self.f = Box::new(f);
    }

    fn set_initial_pt(&mut self, pt: (f64, f64)) {
        self.solved_pts.clear();
        (self.t_curr, self.x_curr) = pt;
        self.solved_pts.push(pt);
    }

    fn _next(&mut self) -> Option<(f64, f64)> {
        let mut y_n = self.x_curr;
        let mut t_n = self.t_curr;
        let mut h = self.h;
        let f = &self.f;

        let k_1 = h * f(t_n, y_n);
        let k_2 = h * f(t_n + C[1]*h, y_n + A[1][0]*k_1);
        let k_3 = h * f(t_n + C[2]*h, y_n + A[2][0]*k_1 + A[2][1]*k_2);
        let k_4 = h * f(t_n + C[3]*h, y_n + A[3][0]*k_1 + A[3][1]*k_2 + A[3][2]*k_3);
        let k_5 = h * f(t_n + C[4]*h, y_n + A[4][0]*k_1 + A[4][1]*k_2 + A[4][2]*k_3 + A[4][3]*k_4);
        let k_6 = h * f(t_n + C[5]*h, y_n + A[5][0]*k_1 + A[5][1]*k_2 + A[5][2]*k_3 + A[5][3]*k_4 + A[5][4]*k_5);
        let k_7 = h * f(t_n + C[6]*h, y_n + A[6][0]*k_1 + A[6][1]*k_2 + A[6][2]*k_3 + A[6][3]*k_4 + A[6][4]*k_5 + A[6][5]*k_6);

        let y_n_1: f64 = y_n + B1[0]*k_1 + B1[2]*k_3 + B1[3]*k_4 + B1[4]*k_5 + B1[5]*k_6;
        // let z_n_1: f64 = y_n + B2[0]*k_1 + B2[2]*k_3 + B2[3]*k_4 + B2[4]*k_5 + B2[5]*k_6 + B2[6]*k_7;
        let mut err = ((B2[0]-B1[0])*k_1 + (B2[2]-B1[2])*k_3 + (B2[3]-B1[3])*k_4 + (B2[4]-B1[4])*k_5
                            + (B2[5]-B1[5])*k_6 + (B2[6]-B1[6])*k_7).abs();

        let h_factor = (f64::EPSILON*h / (2.*err)).powf(1./5.);
        h *= h_factor;
        h = h.clamp(self.h_min, self.h_max);

        println!("err = {}, h_factor = {}, h = {}, self.h = {}", err, h_factor, h, self.h);

        self.t_curr += self.h;
        self.x_curr = y_n_1;
        self.h = h;

        self.solved_pts.push((self.t_curr, self.x_curr));
        Some((self.t_curr, self.x_curr))
    }
}
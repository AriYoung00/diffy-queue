pub trait Solver<'a>: Iterator<Item = (f64, f64)> {
    // fn new(f: F, t_0: f64, x_0: f64, h: f64) -> Self;
    fn solve_at_point(&mut self, t: f64) -> Option<f64>;
    fn set_step_size(&mut self, h: f64);
    fn set_fn(&mut self, f: impl Fn(f64, f64) -> f64 + 'a);
    fn set_initial_pt(&mut self, pt: (f64, f64));
    fn _next(&mut self) -> Option<(f64, f64)>;
}
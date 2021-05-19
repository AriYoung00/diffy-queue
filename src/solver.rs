pub trait Solver: Iterator {
    fn solve_at_point(&mut self, t: f32) -> Option<f32>;
    fn _next(&mut self) -> Option<(f32, f32)>;
}
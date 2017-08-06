/// Matrix Trait
/// 
/// All implementations should follow this structure
pub trait MatrixTrait {
    fn zero(m: usize, n: usize) -> Self;
    fn random(m: usize, n: usize) -> Self;
    fn from_vec(v: &Vec<f64>) -> Self;
    fn generate(m: usize, n: usize, f: &Fn(usize, usize) -> f64) -> Self;
    fn row(&self, n: usize) -> &Vec<f64>;
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn get(&self, m: usize, n: usize) -> f64;
    fn dot(&self, b: &Self) -> Self;
    fn transpose(&self) -> Self;
    fn map(&self, f: &Fn(f64) -> f64) -> Self;
}

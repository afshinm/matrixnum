#[macro_use]
extern crate assert_approx_eq;
extern crate rand;

pub mod basic;
pub mod matrix_trait;

pub struct Matrixnum;

enum Backend {
    Basic,
    Ndarray
}

impl Matrixnum {
    pub fn new(backend: Backend) -> Box<matrix_trait::MatrixTrait> {
        return basic::BasicMatrix;
    }
}

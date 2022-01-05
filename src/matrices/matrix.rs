use std::ops::{Mul};

use array2d::{Array2D, Error};

const EPSILON: f64 = 0.00001;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub grid: Array2D<f64>,
}

impl Matrix {
    pub fn new(num_rows: usize, num_cols: usize) -> Matrix {
        return Matrix {
            grid: Array2D::filled_with(0.0, num_rows, num_cols),
        };
    }

    pub fn from_rows(rows: &Vec<Vec<f64>>) -> Matrix {
        return Matrix {
            grid: Array2D::from_rows(rows),
        };
    }

    pub fn num_rows(&self) -> usize {
        return self.num_rows();
    }

    pub fn num_cols(&self) -> usize {
        return self.num_cols();
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let mut row: usize = 0;
        let mut col: usize = 0;

        for row_iter in other.grid.rows_iter() {
            col = 0;

            for other_number in row_iter {
                let self_number = self.grid[(row, col)];

                if (self_number - other_number).abs() > EPSILON {
                    return false;
                }

                col += 1;
            }
            row += 1;
        }

        return true;
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(&self, rhs: Matrix) -> Result<Self, Error> {
        if (self.num_rows() != rhs.num_cols()) {
            panic!();
        }
        else if (self.num_cols() != rhs.num_rows()) {
            panic!();
        }

        let mut row: usize = 0;
        let mut col: usize = 0;

        for row_iter in other.grid.rows_iter() {
            col = 0;

            for other_number in row_iter {
                let self_number = self.grid[(row, col)];



                col += 1;
            }
            row += 1;
        }

        return Ok();
    }
}
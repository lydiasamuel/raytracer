use std::fmt;
use std::ops::{Mul};

use array2d::{Array2D, Error};

const EPSILON: f64 = 0.00001;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub grid: Box<Array2D<f64>>,
}

impl Matrix {
    pub fn new(num_rows: usize, num_cols: usize) -> Matrix {
        return Matrix {
            width: num_cols,
            height: num_rows,
            grid: Box::new(Array2D::filled_with(0.0, num_rows, num_cols)),
        };
    }

    pub fn from_rows(rows: &Vec<Vec<f64>>) -> Matrix {
        let tmp = Box::new(Array2D::from_rows(rows));

        return Matrix {
            width: tmp.num_cols(),
            height: tmp.num_rows(),
            grid: tmp,
        };
    }

    pub fn num_rows(&self) -> usize {
        return self.height;
    }

    pub fn num_cols(&self) -> usize {
        return self.width;
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        if row >= self.num_rows() {
            panic!();
        }
        if col >= self.num_cols() {
            panic!();
        }

        return self.grid[(row, col)];
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), Error> {
        if row >= self.num_rows() {
            panic!();
        }
        if col >= self.num_cols() {
            panic!();
        }

        self.grid.set(row, col, value);

        return Ok(());
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

    fn mul(self, rhs: Matrix) -> Self {
        if self.num_rows() != rhs.num_cols() {
            panic!();
        }
        else if self.num_cols() != rhs.num_rows() {
            panic!();
        }

        let first_dimension = self.num_rows();
        let second_dimension = rhs.num_cols();
        let shared_dimension = self.num_cols();

        let mut result = Matrix::new(first_dimension, second_dimension);

        for i in 0..first_dimension {
            for j in 0..second_dimension {
                let mut sum: f64 = 0.0;

                for k in 0..shared_dimension {
                    sum = sum + (self.at(i, k) * rhs.at(k, j));
                }

                result.set(i, j, sum);
            }
        }

        return result;
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "".to_string();

        for row_iter in self.grid.rows_iter() {
            for n in row_iter {
                result.push_str(format!("{}, ", n).as_str());
            }
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}
use crate::Vector;
use crate::Point;
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

pub struct MatrixOperationDimensionError;

impl fmt::Display for MatrixOperationDimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix dimensions error, please ensure all dimensions are compatible or within bounds!")
    }
}

impl fmt::Debug for MatrixOperationDimensionError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
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
        let tmp = Array2D::from_rows(rows);

        return Matrix {
            width: tmp.num_columns(),
            height: tmp.num_rows(),
            grid: Box::new(tmp),
        };
    }

    pub fn identity(size: usize) -> Matrix {
        let mut tmp = Array2D::filled_with(0.0, size, size);

        for i in 0..size {
            let fill = tmp.set(i, i, 1.0);

            match fill {
                Ok(()) => {},
                Err(error) => panic!("Identity could not be created: {:?}", error)
            }
        }

        return Matrix {
            width: tmp.num_columns(),
            height: tmp.num_rows(),
            grid: Box::new(tmp),
        };
    }

    // These ampersands represent references, and they allow you to refer to some value without taking ownership of it.
    pub fn transpose(&self) -> Matrix {
        // Swap the dimensions around
        let mut result = Matrix::new(self.num_cols(), self.num_rows());

        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                let fill = result.set(j, i, self.at(i, j));

                match fill {
                    Ok(()) => {},
                    Err(error) => panic!("Transposition could not be created: {:?}", error)
                }
            }
        }

        return result;
    }

    pub fn invertable(&self) -> bool {
        return ((self.determinant()).abs() - 0.0) > EPSILON;
    }

    pub fn inverse(&self) -> Matrix {
        let rows = self.num_rows();
        let cols = self.num_cols();
        let det = self.determinant();
        let mut result = Matrix::new(rows, cols);

        if self.invertable() {
            for row in 0..rows {
                for col in 0..cols {
                    let cofactor = self.cofactor(row, col);

                    let cofactor = match cofactor {
                        Ok(result) => result,
                        Err(error) => panic!("Cofactor for inverse could not be calculated: {:?}", error)
                    };

                    let fill = result.set(col, row, cofactor / det);

                    match fill {
                        Ok(()) => {},
                        Err(error) => panic!("Inverse could not be created: {:?}", error)
                    }
                }
            }
        }
        
        return result;
    }

    // Works with 2x2 Matrices
    pub fn determinant(&self) -> f64 {
        if self.num_rows() == 2 && self.num_cols() == 2 {
            let a = self.at(0, 0);
            let b = self.at(0, 1);
            let c = self.at(1, 0);
            let d = self.at(1, 1);
    
            return a * d - b * c;
        }
        else {
            let mut det = 0.0;

            for col in 0..self.num_cols() {
                let cofactor = self.cofactor(0, col);

                let cofactor = match cofactor {
                    Ok(result) => result,
                    Err(error) => panic!("Cofactor for determinant could not be calculated: {:?}", error)
                };

                det = det + (self.at(0, col) * cofactor);
            }

            return det;
        }
    }

    // Works with 4x4 and 3x3 matrices 
    fn cofactor(&self, row: usize, col: usize) -> Result<f64, MatrixOperationDimensionError>{
        if self.num_rows() == 4 && self.num_cols() == 4 {
            let base: f64 = -1.0;
            let exp: f64 = (row + col) as f64;

            let sub = self.submatrix(row, col);
            let det = sub.determinant();

            return Ok(base.powf(exp) * det);
        } 
        else if self.num_rows() == 3 && self.num_cols() == 3 {
            if (row + col) % 2 != 0 {
                return Ok(-self.minor(row, col));
            }
            else {
                return Ok(self.minor(row, col));
            }
        } 
        else {
            return Err(MatrixOperationDimensionError);
        }
    }

    // Works with 3x3 Matrices
    fn minor(&self, row: usize, col: usize) -> f64 {        
        let sub = self.submatrix(row, col);

        return sub.determinant();
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut result = Matrix::new(self.num_rows() - 1, self.num_cols() - 1);

        let mut new_row = 0;
        let mut new_col = 0;

        for i in 0..self.num_rows() {
            if i != row {
                for j in 0..self.num_cols() {
                    if j != col {
                        let fill = result.set(new_row, new_col, self.at(i, j));

                        match fill {
                            Ok(()) => {},
                            Err(error) => panic!("Submatrix could not be created: {:?}", error)
                        }

                        new_col = new_col + 1;
                    }
                }

                new_col = 0;
                new_row = new_row + 1;
            }
        }

        return result;
    }

    pub fn num_rows(&self) -> usize {
        return self.height;
    }

    pub fn num_cols(&self) -> usize {
        return self.width;
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        return self.grid[(row, col)];
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), Error> {
        self.grid.set(row, col, value)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let mut col: usize;
        let mut row: usize;

        row = 0;
        
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
    type Output = Result<Self, MatrixOperationDimensionError>;

    fn mul(self, rhs: Matrix) -> Result<Self, MatrixOperationDimensionError> {
        if self.num_cols() != rhs.num_rows() {
            return Err(MatrixOperationDimensionError)
        }

        let first_dimension = self.num_rows();
        let second_dimension = rhs.num_cols();
        let shared_dimension = self.num_cols();

        let mut result = Matrix::new(first_dimension, second_dimension);

        // Matrix multiplication computes the dot product of every row-column combination in the two matrices!
        for row in 0..first_dimension {
            for col in 0..second_dimension {
                let mut sum: f64 = 0.0;

                for k in 0..shared_dimension {
                    sum = sum + (self.at(row, k) * rhs.at(k, col));
                }

                let fill = result.set(row, col, sum);

                match fill {
                    Ok(()) => {},
                    Err(error) => panic!("Matrix multiplication could not be performed: {:?}", error)
                }
            }
        }

        return Ok(result);
    }
}

impl Mul<Point> for Matrix {
    type Output = Result<Point, MatrixOperationDimensionError>;

    fn mul(self, rhs: Point) -> Result<Point, MatrixOperationDimensionError> {
        if self.num_cols() != 4 || self.num_rows() != 4 {
            return Err(MatrixOperationDimensionError)
        }

        let mut tup = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            tup[i] = self.at(i, 0) * rhs.x + 
                self.at(i, 1) * rhs.y + 
                self.at(i, 2) * rhs.z + 
                self.at(i, 3) * rhs.w;
        }

        return Ok(Point::construct(tup[0], tup[1], tup[2], tup[3]));
    }
}

impl Mul<Vector> for Matrix {
    type Output = Result<Vector, MatrixOperationDimensionError>;

    fn mul(self, rhs: Vector) -> Result<Vector, MatrixOperationDimensionError> {
        if self.num_cols() != 4 || self.num_rows() != 4 {
            return Err(MatrixOperationDimensionError);
        }

        let mut tup = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            tup[i] = self.at(i, 0) * rhs.x + 
                self.at(i, 1) * rhs.y + 
                self.at(i, 2) * rhs.z + 
                self.at(i, 3) * rhs.w;
        }

        return Ok(Vector::construct(tup[0], tup[1], tup[2], tup[3]));
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "".to_string();

        for row_iter in self.grid.rows_iter() {
            for n in row_iter {
                result.push_str(format!("{} ", n).as_str());
            }
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}
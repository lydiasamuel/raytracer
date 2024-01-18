use std::ops::Mul;

use {array2d::Array2D, array2d::Error};

use crate::tuples::tuple::Tuple;

use crate::EPSILON;

#[derive(Debug, Clone)]
pub struct Matrix {
    grid: Array2D<f64>,
}

impl Matrix {
    pub fn new(num_rows: usize, num_columns: usize) -> Matrix {
        if num_rows == 0 || num_columns == 0 {
            panic!("Error: Unable to make matrix with zero sized dimensions")
        }

        return Matrix {
            grid: Array2D::filled_with(0.0, num_rows, num_columns)
        };
    }

    pub fn identity(size: usize) -> Matrix {
        if size == 0 {
            panic!("Error: Unable to make zero sized identity matrix")
        }

        let mut grid = Array2D::filled_with(0.0, size, size);

        for i in 0..size {
            let fill = grid.set(i, i, 1.0);

            match fill {
                Ok(()) => {},
                _ => panic!("Error: Unable to write to result of matrix multiplication during initial calculation")
            }
        }

        return Matrix {
            grid
        };
    }

    pub fn from_columns(columns: &Vec<Vec<f64>>) -> Result<Matrix, array2d::Error> {
        let grid = Array2D::from_columns(columns)?;

        return Ok(Matrix {
            grid
        });
    }

    pub fn from_rows(rows: &Vec<Vec<f64>>) -> Result<Matrix, array2d::Error>{
        let grid = Array2D::from_rows(rows)?;

        return Ok(Matrix {
            grid
        });
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.num_columns(), self.num_rows());

        for row in 0..self.num_rows() {
            for column in 0..self.num_columns() {
                let value = *self.get(row, column).unwrap();
                let fill = result.set(column, row, value);

                match fill {
                    Ok(()) => {},
                    _ => panic!("Error: Unable to write to result of matrix transposition during construction")
                }
            }
        }

        return result;
    }

    /* The determinant is a number that is derived from the elements of a matrix. 
     * The name comes from the use of matrices to solve systems of equations, where it's used to
     * determine whether or not the system has a solution. If the determinant is zero, then the
     * corresponding system of equations has no solution.
     */
    pub fn determinant(&self) -> f64 {
        if self.num_rows() == 2 && self.num_columns() == 2 {
            let a = *self.get(0, 0).unwrap();
            let b = *self.get(0, 1).unwrap();
            let c = *self.get(1, 0).unwrap();
            let d = *self.get(1, 1).unwrap();
    
            return a * d - b * c;
        }
        /*else {
            let mut determinant = 0.0;

            for column in 0..self.num_columns() {
                let cofactor = self.cofactor(0, col);

                let cofactor = match cofactor {
                    Ok(result) => result,
                    _ => panic!("Error: Cofactor for determinant could not be calculated")
                };

                determinant += *self.get(0, column).unwrap() * cofactor;
            }

            return determinant;
        }*/

        return 0.0;
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Result<Matrix, &'static str> {
        if self.num_rows() == 1 || self.num_columns() == 1 {
            return Err("Error: Unable to take submatrix since both dimensions are not greater than one");
        }

        let mut result = Matrix::new(self.num_rows() - 1, self.num_columns() - 1);
        let mut result_y = 0;
        let mut result_x = 0;

        for y in 0..self.num_rows() {

            if y != row {
                for x in 0..self.num_columns() {

                    if x != column {
                        let value = *self.get(y, x).unwrap();

                        let fill = result.set(result_y, result_x, value);

                        match fill {
                            Ok(()) => {},
                            _ => panic!("Error: Unable to write to resulting submatrix during construction")
                        }
                        
                        result_x += 1;
                    }
                }

                result_x = 0;
                result_y += 1;
            }  
        }

        return Ok(result);
    }

    fn minor(&self, row: usize, col: usize) -> Result<f64, &'static str>  {     
        if self.num_rows() != 3 || self.num_columns() != 3 {
            return Err("Error: Unable to take minor since both dimensions are not equal to three");
        }

        let sub = self.submatrix(row, col)?;

        return Ok(sub.determinant());
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&f64> {
        return self.grid.get(row, column);
    }

    pub fn set(&mut self, row: usize, column: usize, element: f64) ->  Result<(), Error> {
        return self.grid.set(row, column, element);
    }

    pub fn num_rows(&self) -> usize {
        return self.grid.num_rows();
    }

    pub fn num_columns(&self) -> usize {
        return self.grid.num_columns();
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Result<Tuple, &'static str>;

    fn mul(self, rhs: Tuple) -> Result<Tuple, &'static str> {
        if self.num_columns() != 4 || self.num_rows() != 4 {
            return Err("Error: Incompatible matrix-tuple sizes for multiplication")
        }

        let mut result = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            result[i] = self.get(i, 0).unwrap() * rhs.x + 
                self.get(i, 1).unwrap() * rhs.y + 
                self.get(i, 2).unwrap() * rhs.z + 
                self.get(i, 3).unwrap() * rhs.w;
        }

        return Ok(Tuple::new(result[0], result[1], result[2], result[3]));
    }
}

impl Mul for Matrix {
    type Output = Result<Self, &'static str>;

    fn mul(self, rhs: Matrix) -> Result<Self, &'static str> {
        if self.num_columns() != rhs.num_rows() {
            return Err("Error: Incompatible matrix sizes for multiplication");
        }

        let first_dimension = self.num_rows();
        let second_dimension = rhs.num_columns();
        let shared_dimension = self.num_columns();

        let mut result = Matrix::new(first_dimension, second_dimension);

        // Matrix multiplication computes the dot product of every row-column combination in the two matrices!
        for row in 0..first_dimension {
            for col in 0..second_dimension {
                let mut sum: f64 = 0.0;

                for k in 0..shared_dimension {
                    sum = sum + (self.get(row, k).unwrap() * rhs.get(k, col).unwrap());
                }

                let fill = result.set(row, col, sum);

                match fill {
                    Ok(()) => {},
                    _ => panic!("Error: Unable to write to result of matrix multiplication during initial calculation")
                }
            }
        }

        return Ok(result);
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.num_rows() != other.num_rows() {
            return false;
        }

        if self.num_columns() != other.num_columns() {
            return false;
        }

        let mut iter = self.grid.elements_row_major_iter();
        let mut other_iter = other.grid.elements_row_major_iter();

        while let Some(num) = iter.next() {
            if let Some(other_num) = other_iter.next() {
                if (num - other_num).abs() > EPSILON {
                    return false;
                }
            }
            else {
                panic!("Error: Unexpected end to other matrix in comparison.")
            }
        } 
      
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_normal_values_for_a_matrix_when_creating_a_4_by_4_should_instantiate_correctly() {
        let rows = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        assert_eq!(1.0, *matrix.get(0, 0).unwrap());
        assert_eq!(4.0, *matrix.get(0, 3).unwrap());
        assert_eq!(5.5, *matrix.get(1, 0).unwrap());
        assert_eq!(7.5, *matrix.get(1, 2).unwrap());
        assert_eq!(11.0, *matrix.get(2, 2).unwrap());
        assert_eq!(13.5, *matrix.get(3, 0).unwrap());
        assert_eq!(15.5, *matrix.get(3, 2).unwrap());
    }

    #[test]
    fn given_normal_values_for_a_matrix_when_creating_a_3_by_3_should_instantiate_correctly() {
        let rows = vec![
            vec![-3.0, 5.0, 0.0], 
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        assert_eq!(-3.0, *matrix.get(0, 0).unwrap());
        assert_eq!(-2.0, *matrix.get(1, 1).unwrap());
        assert_eq!(1.0, *matrix.get(2, 2).unwrap());
    }

    #[test]
    fn given_normal_values_for_a_matrix_when_creating_a_2_by_2_should_instantiate_correctly() {
        let rows = vec![
            vec![-3.0, 5.0], 
            vec![1.0, -2.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        assert_eq!(-3.0, *matrix.get(0, 0).unwrap());
        assert_eq!(5.0, *matrix.get(0, 1).unwrap());
        assert_eq!(1.0, *matrix.get(1, 0).unwrap());
        assert_eq!(-2.0, *matrix.get(1, 1).unwrap());
    }

    #[test]
    fn given_two_equal_matrices_when_comparing_them_should_return_true() {
        let rows = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];

        let matrix_a = Matrix::from_rows(&rows).unwrap();
        let matrix_b = Matrix::from_rows(&rows).unwrap();

        assert_eq!(true, matrix_a == matrix_b);
    }

    #[test]
    fn given_two_unequal_matrices_when_comparing_them_should_return_false() {
        let rows_a = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];

        let rows_b = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 10.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];

        let matrix_a = Matrix::from_rows(&rows_a).unwrap();
        let matrix_b = Matrix::from_rows(&rows_b).unwrap();

        assert_eq!(false, matrix_a == matrix_b);
    }

    #[test]
    fn given_two_unequal_matrices_in_size_when_comparing_them_should_return_false() {
        let rows_a = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0]
        ];

        let rows_b = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 10.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];

        let matrix_a = Matrix::from_rows(&rows_a).unwrap();
        let matrix_b = Matrix::from_rows(&rows_b).unwrap();

        assert_eq!(false, matrix_a == matrix_b);
    }

    #[test]
    fn given_two_matrices_when_multiplying_them_should_correctly_calculate_the_result() {
        let rows_a = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]
        ];

        let rows_b = vec![
            vec![-2.0, 1.0, 2.0, 3.0], 
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0]
        ];

        let expected_rows = vec![
            vec![20.0, 22.0, 50.0, 48.0], 
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0]
        ];

        let matrix_a = Matrix::from_rows(&rows_a).unwrap();
        let matrix_b = Matrix::from_rows(&rows_b).unwrap();

        let expected = Matrix::from_rows(&expected_rows).unwrap();
        let result = matrix_a * matrix_b;

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_matrix_and_a_tuple_when_multiplying_them_should_correctly_calculate_the_result() {
        let rows = vec![
            vec![1.0, 2.0, 3.0, 4.0], 
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);

        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);

        let result = matrix * tuple;

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_matrix_and_the_identity_when_multiplying_them_should_just_return_the_former() {
        let rows = vec![
            vec![0.0, 1.0, 2.0, 4.0], 
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();
        let identity = Matrix::identity(4);

        let expected = Matrix::from_rows(&rows).unwrap();
        let result = matrix * identity;

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_matrix_when_transposing_it_should_correctly_return_result() {
        let rows = vec![
            vec![0.0, 9.0, 3.0, 0.0], 
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0]
        ];
        
        let expected_rows = vec![
            vec![0.0, 9.0, 1.0, 0.0], 
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();
        let expected = Matrix::from_rows(&expected_rows).unwrap();

        let result = matrix.transpose();

        assert_eq!(expected, result);
    }

    #[test]
    fn given_the_identity_matrix_when_transposing_it_should_return_itself() {
        let matrix = Matrix::identity(4);

        let result = matrix.transpose();

        assert_eq!(matrix, result);
    }

    #[test]
    fn given_a_2_by_2_matrix_when_taking_the_determinant_should_correctly_calculate_result() {
        let rows = vec![
            vec![1.0, 5.0], 
            vec![-3.0, 2.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.determinant();

        assert_eq!(17.0, result);
    }

    #[test]
    fn given_a_3_by_3_matrix_when_taking_a_submatrix_should_output_a_2_by_2_matrix() {
        let rows = vec![
            vec![1.0, 5.0, 0.0], 
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0]
        ];

        let expected_rows = vec![
            vec![-3.0, 2.0], 
            vec![0.0, 6.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();
        let expected = Matrix::from_rows(&expected_rows).unwrap();

        let result = matrix.submatrix(0, 2).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn given_a_4_by_4_matrix_when_taking_a_submatrix_should_output_a_3_by_3_matrix() {
        let rows = vec![
            vec![-6.0, 1.0, 1.0, 6.0], 
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0]
        ];

        let expected_rows = vec![
            vec![-6.0, 1.0, 6.0], 
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();
        let expected = Matrix::from_rows(&expected_rows).unwrap();

        let result = matrix.submatrix(2, 1).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn given_a_3_by_3_matrix_when_taking_a_minor_should_output_determinant_of_the_2_by_2_matrix() {
        let rows = vec![
            vec![3.0, 5.0, 0.0], 
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0]
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.minor(1, 0).unwrap();

        assert_eq!(25.0, result);
    }
}
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
            grid: Array2D::filled_with(0.0, num_rows, num_columns),
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

        return Matrix { grid };
    }

    pub fn from_columns(columns: &Vec<Vec<f64>>) -> Result<Matrix, array2d::Error> {
        let grid = Array2D::from_columns(columns)?;

        return Ok(Matrix { grid });
    }

    pub fn from_rows(rows: &Vec<Vec<f64>>) -> Result<Matrix, array2d::Error> {
        let grid = Array2D::from_rows(rows)?;

        return Ok(Matrix { grid });
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let transform = Matrix::from_rows(&vec![
            vec![1.0, 0.0, 0.0, x],
            vec![0.0, 1.0, 0.0, y],
            vec![0.0, 0.0, 1.0, z],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        return transform.unwrap();
    }

    // Side note: Reflection is just scaling by a negative value along a certain axis
    pub fn reflect_x() -> Matrix {
        return Matrix::scaling(-1.0, 1.0, 1.0);
    }

    pub fn reflect_y() -> Matrix {
        return Matrix::scaling(1.0, -1.0, 1.0);
    }

    pub fn reflect_z() -> Matrix {
        return Matrix::scaling(1.0, 1.0, -1.0);
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let transform = Matrix::from_rows(&vec![
            vec![x, 0.0, 0.0, 0.0],
            vec![0.0, y, 0.0, 0.0],
            vec![0.0, 0.0, z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        return transform.unwrap();
    }

    pub fn rotation_x(radians: f64) -> Matrix {
        let transform = Matrix::from_rows(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, radians.cos(), -radians.sin(), 0.0],
            vec![0.0, radians.sin(), radians.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        return transform.unwrap();
    }

    pub fn rotation_y(radians: f64) -> Matrix {
        let transform = Matrix::from_rows(&vec![
            vec![radians.cos(), 0.0, radians.sin(), 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-radians.sin(), 0.0, radians.cos(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        return transform.unwrap();
    }

    pub fn rotation_z(radians: f64) -> Matrix {
        let transform = Matrix::from_rows(&vec![
            vec![radians.cos(), -radians.sin(), 0.0, 0.0],
            vec![radians.sin(), radians.cos(), 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        return transform.unwrap();
    }

    /* A shearing transformation changes each component of the tuple in proportion to the
     * other two components. So the x component changes in proportion to the y and z, y changes
     * in proportion to x and z, and z changes in proportion to x and y.
     */
    pub fn shearing(x2y: f64, x2z: f64, y2x: f64, y2z: f64, z2x: f64, z2y: f64) -> Matrix {
        let transform = Matrix::from_rows(&vec![
            vec![1.0, x2y, x2z, 0.0],
            vec![y2x, 1.0, y2z, 0.0],
            vec![z2x, z2y, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        return transform.unwrap();
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

    /* Inversion is the operation that allows you to reverse the effect of multiplying by a matrix.
     * Not every matrix is invertible, if the determinant of the matrix is 0 then there is no inverse.
     * Method here is the matrix of cofactors
     *   1. Create a matrix that that consists of the cofactors of each of the original elements
     *   2. Transpose the cofactor matrix
     *   3. Divide each of the resulting elements by the determinant of the original matrix
     */
    pub fn inverse(&self) -> Result<Matrix, &'static str> {
        let determinant = self.determinant()?;

        if (determinant.abs() - 0.0) < EPSILON {
            return Err("Error: Matrix is not invertable due to a zero determinant");
        }

        let mut result = Matrix::new(self.num_rows(), self.num_columns());

        for row in 0..self.num_rows() {
            for column in 0..self.num_columns() {
                let cofactor = self.cofactor(row, column)?;

                let fill = result.set(column, row, cofactor / determinant);

                match fill {
                    Ok(()) => {}
                    _ => panic!(
                        "Error: Unable to write to result of matrix inversion during construction"
                    ),
                }
            }
        }

        return Ok(result);
    }

    /* The determinant is a number that is derived from the elements of a matrix.
     * The name comes from the use of matrices to solve systems of equations, where it's used to
     * determine whether or not the system has a solution. If the determinant is zero, then the
     * corresponding system of equations has no solution.
     */
    pub fn determinant(&self) -> Result<f64, &'static str> {
        if self.num_rows() == 2 && self.num_columns() == 2 {
            let a = *self.get(0, 0).unwrap();
            let b = *self.get(0, 1).unwrap();
            let c = *self.get(1, 0).unwrap();
            let d = *self.get(1, 1).unwrap();

            return Ok(a * d - b * c);
        } else {
            let mut determinant = 0.0;

            for column in 0..self.num_columns() {
                let cofactor = self.cofactor(0, column)?;
                determinant += *self.get(0, column).unwrap() * cofactor;
            }

            return Ok(determinant);
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Result<Matrix, &'static str> {
        if self.num_rows() == 1 || self.num_columns() == 1 {
            return Err(
                "Error: Unable to take submatrix since both dimensions are not greater than one",
            );
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
                            Ok(()) => {}
                            _ => panic!(
                                "Error: Unable to write to resulting submatrix during construction"
                            ),
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

    // A minor of an element at row i and column j is the determinant of the submatrix at (i, j)
    fn minor(&self, row: usize, column: usize) -> Result<f64, &'static str> {
        if self.num_rows() == 3 && self.num_columns() == 3 {
            let submatrix = self.submatrix(row, column)?;

            let determinant = submatrix.determinant()?;

            return Ok(determinant);
        } else {
            return Err("Error: Unable to take minor since both dimensions are not equal to three");
        }
    }

    // A cofactor is a minor that possibly has their sign changed
    fn cofactor(&self, row: usize, column: usize) -> Result<f64, &'static str> {
        if self.num_rows() == 4 && self.num_columns() == 4 {
            let base: f64 = -1.0;
            let exp: f64 = (row + column) as f64;

            let sub = self.submatrix(row, column).unwrap();
            let det = sub.determinant()?;

            return Ok(base.powf(exp) * det);
        } else if self.num_rows() == 3 && self.num_columns() == 3 {
            let result = self.minor(row, column)?;

            if (row + column) % 2 != 0 {
                return Ok(-result);
            } else {
                return Ok(result);
            }
        } else {
            return Err("Error: This function can only take cofactor of a 4x4 or a 3x3 matrix");
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&f64> {
        return self.grid.get(row, column);
    }

    pub fn set(&mut self, row: usize, column: usize, element: f64) -> Result<(), Error> {
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
            return Err("Error: Incompatible matrix-tuple sizes for multiplication");
        }

        let mut result = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            result[i] = self.get(i, 0).unwrap() * rhs.x
                + self.get(i, 1).unwrap() * rhs.y
                + self.get(i, 2).unwrap() * rhs.z
                + self.get(i, 3).unwrap() * rhs.w;
        }

        return Ok(Tuple::new(result[0], result[1], result[2], result[3]));
    }
}

// Matrix multiplication is associative, but not commutative. A x B is not the same as B x A.
// You must concatenate the transformations in the reverse order to have them applied in the order you want!
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
            } else {
                panic!("Error: Unexpected end to other matrix in comparison.")
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts;

    #[test]
    fn given_normal_values_for_a_matrix_when_creating_a_4_by_4_should_instantiate_correctly() {
        let rows = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
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
            vec![0.0, 1.0, 1.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        assert_eq!(-3.0, *matrix.get(0, 0).unwrap());
        assert_eq!(-2.0, *matrix.get(1, 1).unwrap());
        assert_eq!(1.0, *matrix.get(2, 2).unwrap());
    }

    #[test]
    fn given_normal_values_for_a_matrix_when_creating_a_2_by_2_should_instantiate_correctly() {
        let rows = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];

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
            vec![13.5, 14.5, 15.5, 16.5],
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
            vec![13.5, 14.5, 15.5, 16.5],
        ];

        let rows_b = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 10.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
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
            vec![9.0, 10.0, 11.0, 12.0],
        ];

        let rows_b = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 10.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
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
            vec![5.0, 4.0, 3.0, 2.0],
        ];

        let rows_b = vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ];

        let expected_rows = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
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
            vec![0.0, 0.0, 0.0, 1.0],
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
            vec![4.0, 8.0, 16.0, 32.0],
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
            vec![0.0, 0.0, 5.0, 8.0],
        ];

        let expected_rows = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
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
        let rows = vec![vec![1.0, 5.0], vec![-3.0, 2.0]];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.determinant().unwrap();

        assert_eq!(17.0, result);
    }

    #[test]
    fn given_a_3_by_3_matrix_when_taking_the_determinant_should_correctly_calculate_result() {
        let rows = vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.cofactor(0, 0).unwrap();
        assert_eq!(56.0, result);

        let result = matrix.cofactor(0, 1).unwrap();
        assert_eq!(12.0, result);

        let result = matrix.cofactor(0, 2).unwrap();
        assert_eq!(-46.0, result);

        let result = matrix.determinant().unwrap();
        assert_eq!(-196.0, result);
    }

    #[test]
    fn given_a_4_by_4_matrix_when_taking_the_determinant_should_correctly_calculate_result() {
        let rows = vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.cofactor(0, 0).unwrap();
        assert_eq!(690.0, result);

        let result = matrix.cofactor(0, 1).unwrap();
        assert_eq!(447.0, result);

        let result = matrix.cofactor(0, 2).unwrap();
        assert_eq!(210.0, result);

        let result = matrix.cofactor(0, 3).unwrap();
        assert_eq!(51.0, result);

        let result = matrix.determinant().unwrap();
        assert_eq!(-4071.0, result);
    }

    #[test]
    fn given_a_3_by_3_matrix_when_taking_a_submatrix_should_output_a_2_by_2_matrix() {
        let rows = vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ];

        let expected_rows = vec![vec![-3.0, 2.0], vec![0.0, 6.0]];

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
            vec![-7.0, 1.0, -1.0, 1.0],
        ];

        let expected_rows = vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
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
            vec![6.0, -1.0, 5.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.minor(1, 0).unwrap();

        assert_eq!(25.0, result);
    }

    #[test]
    fn given_a_3_by_3_matrix_when_taking_a_cofactor_should_output_correctly_signed_minor() {
        let rows = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.minor(0, 0).unwrap();
        assert_eq!(-12.0, result);

        let result = matrix.cofactor(0, 0).unwrap();
        assert_eq!(-12.0, result);

        let result = matrix.minor(1, 0).unwrap();
        assert_eq!(25.0, result);

        let result = matrix.cofactor(1, 0).unwrap();
        assert_eq!(-25.0, result);
    }

    #[test]
    fn given_a_non_invertible_4_by_4_matrix_when_taking_the_inversion_should_output_error_result() {
        let rows = vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let result = matrix.inverse();

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn given_an_invertible_4_by_4_matrix_when_taking_the_inversion_should_output_correct_result() {
        let rows = vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();

        let determinant = matrix.determinant().unwrap();
        let inverse = matrix.inverse().unwrap();

        assert_eq!(532.0, determinant);

        let result = matrix.cofactor(2, 3).unwrap();
        assert_eq!(-160.0, result);

        let result = *inverse.get(3, 2).unwrap();
        let expected = -160.0 / determinant;
        assert_eq!(expected, result);

        let result = matrix.cofactor(3, 2).unwrap();
        assert_eq!(105.0, result);

        let result = *inverse.get(2, 3).unwrap();
        let expected = 105.0 / determinant;
        assert_eq!(expected, result);

        let expected_rows = vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ];

        let expected_inverse = Matrix::from_rows(&expected_rows).unwrap();

        assert_eq!(expected_inverse, inverse);
    }

    #[test]
    fn given_a_couple_invertible_4_by_4_matrices_when_taking_the_inversion_should_output_correct_result(
    ) {
        let rows = vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();
        let inverse = matrix.inverse().unwrap();

        let expected_rows = vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ];

        let expected_inverse = Matrix::from_rows(&expected_rows).unwrap();

        assert_eq!(expected_inverse, inverse);

        let rows = vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ];

        let matrix = Matrix::from_rows(&rows).unwrap();
        let inverse = matrix.inverse().unwrap();

        let expected_rows = vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ];

        let expected_inverse = Matrix::from_rows(&expected_rows).unwrap();

        assert_eq!(expected_inverse, inverse);
    }

    #[test]
    fn given_two_4_by_4_matrices_when_multiplying_the_product_by_the_inverse_of_the_latter_should_output_the_former(
    ) {
        let rows_a = vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ];

        let rows_b = vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ];

        let matrix_a = Matrix::from_rows(&rows_a).unwrap();
        let matrix_b = Matrix::from_rows(&rows_b).unwrap();
        let inverse = matrix_b.inverse().unwrap();

        let product = matrix_a * matrix_b;
        let result = product.unwrap() * inverse;

        let expected = Matrix::from_rows(&rows_a).unwrap();

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_translation_matrix_when_multiplying_them_should_move_the_point_by_the_given_amount(
    ) {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let point = Tuple::point(-3.0, 4.0, 5.0);

        let result = transform * point;
        let expected = Tuple::point(2.0, 1.0, 7.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_an_inverse_translation_matrix_when_multiplying_them_should_move_the_point_by_the_given_amount_in_reverse(
    ) {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse().unwrap();
        let point = Tuple::point(-3.0, 4.0, 5.0);

        let result = inverse * point;
        let expected = Tuple::point(-8.0, 7.0, 3.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_vector_and_a_translation_matrix_when_multiplying_them_should_not_change_the_vector()
    {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let vector = Tuple::vector(-3.0, 4.0, 5.0);

        let result = transform * vector;

        assert_eq!(vector, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_scaling_matrix_when_multiplying_them_should_scale_the_point_correctly() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let point = Tuple::point(-4.0, 6.0, 8.0);

        let result = transform * point;
        let expected = Tuple::point(-8.0, 18.0, 32.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_vector_and_a_scaling_matrix_when_multiplying_them_should_scale_the_vector_correctly()
    {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        let result = transform * vector;
        let expected = Tuple::vector(-8.0, 18.0, 32.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_vector_and_an_inverse_scaling_matrix_when_multiplying_them_should_scale_correctly_in_the_opposite_way(
    ) {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inverse = transform.inverse().unwrap();
        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        let result = inverse * vector;
        let expected = Tuple::vector(-2.0, 2.0, 2.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_reflection_matrix_when_multiplying_them_should_reflect_the_point_correctly(
    ) {
        let transform = Matrix::reflect_x();
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(-2.0, 3.0, 4.0);

        assert_eq!(expected, result.unwrap());

        let transform = Matrix::reflect_y();
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(2.0, -3.0, 4.0);

        assert_eq!(expected, result.unwrap());

        let transform = Matrix::reflect_z();
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(2.0, 3.0, -4.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_an_x_axis_rotation_matrix_when_multiplying_them_should_rotate_the_point_correctly(
    ) {
        let half_quarter = Matrix::rotation_x(consts::PI / 4.0);
        let full_quarter = Matrix::rotation_x(consts::PI / 2.0);

        let point = Tuple::point(0.0, 1.0, 0.0);

        let result = half_quarter * point;
        let expected = Tuple::point(0.0, consts::SQRT_2 / 2.0, consts::SQRT_2 / 2.0);

        assert_eq!(expected, result.unwrap());

        let result = full_quarter * point;
        let expected = Tuple::point(0.0, 0.0, 1.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_an_inverse_x_axis_rotation_matrix_when_multiplying_them_should_rotate_the_point_correctly(
    ) {
        let half_quarter = Matrix::rotation_x(consts::PI / 4.0);
        let inverse = half_quarter.inverse().unwrap();

        let point = Tuple::point(0.0, 1.0, 0.0);

        let result = inverse * point;
        let expected = Tuple::point(0.0, consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_y_axis_rotation_matrix_when_multiplying_them_should_rotate_the_point_correctly(
    ) {
        let half_quarter = Matrix::rotation_y(consts::PI / 4.0);
        let full_quarter = Matrix::rotation_y(consts::PI / 2.0);

        let point = Tuple::point(0.0, 0.0, 1.0);

        let result = half_quarter * point;
        let expected = Tuple::point(consts::SQRT_2 / 2.0, 0.0, consts::SQRT_2 / 2.0);

        assert_eq!(expected, result.unwrap());

        let result = full_quarter * point;
        let expected = Tuple::point(1.0, 0.0, 0.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_z_axis_rotation_matrix_when_multiplying_them_should_rotate_the_point_correctly(
    ) {
        let half_quarter = Matrix::rotation_z(consts::PI / 4.0);
        let full_quarter = Matrix::rotation_z(consts::PI / 2.0);

        let point = Tuple::point(0.0, 1.0, 0.0);

        let result = half_quarter * point;
        let expected = Tuple::point(-consts::SQRT_2 / 2.0, consts::SQRT_2 / 2.0, 0.0);

        assert_eq!(expected, result.unwrap());

        let result = full_quarter * point;
        let expected = Tuple::point(-1.0, 0.0, 0.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_x2y_only_shearing_matrix_when_multiplying_them_should_move_the_point_correctly(
    ) {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(5.0, 3.0, 4.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_x2z_only_shearing_matrix_when_multiplying_them_should_move_the_point_correctly(
    ) {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(6.0, 3.0, 4.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_y2x_only_shearing_matrix_when_multiplying_them_should_move_the_point_correctly(
    ) {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(2.0, 5.0, 4.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_y2z_only_shearing_matrix_when_multiplying_them_should_move_the_point_correctly(
    ) {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(2.0, 7.0, 4.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_z2x_only_shearing_matrix_when_multiplying_them_should_move_the_point_correctly(
    ) {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(2.0, 3.0, 6.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_a_point_and_a_z2y_only_shearing_matrix_when_multiplying_them_should_move_the_point_correctly(
    ) {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * point;
        let expected = Tuple::point(2.0, 3.0, 7.0);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn given_several_transformation_matrices_when_applied_individualy_in_sequence_should_transform_point_correctly(
    ) {
        let transform_a = Matrix::rotation_x(consts::PI / 2.0);
        let transform_b = Matrix::scaling(5.0, 5.0, 5.0);
        let transform_c = Matrix::translation(10.0, 5.0, 7.0);

        let point = Tuple::point(1.0, 0.0, 1.0);

        let result = (transform_a * point).unwrap();
        let expected = Tuple::point(1.0, -1.0, 0.0);

        assert_eq!(expected, result);

        let result = (transform_b * result).unwrap();
        let expected = Tuple::point(5.0, -5.0, 0.0);

        assert_eq!(expected, result);

        let result = (transform_c * result).unwrap();
        let expected = Tuple::point(15.0, 0.0, 7.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_several_transformation_matrices_when_chained_in_reverse_order_should_transform_point_correctly(
    ) {
        let transform_a = Matrix::rotation_x(consts::PI / 2.0);
        let transform_b = Matrix::scaling(5.0, 5.0, 5.0);
        let transform_c = Matrix::translation(10.0, 5.0, 7.0);

        let point = Tuple::point(1.0, 0.0, 1.0);

        let result =
            (((transform_c * transform_b).unwrap() * transform_a).unwrap() * point).unwrap();
        let expected = Tuple::point(15.0, 0.0, 7.0);

        assert_eq!(expected, result);
    }
}

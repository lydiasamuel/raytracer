use {array2d::Array2D, array2d::Error};

#[derive(Debug, Clone)]
pub struct Matrix {
    grid: Array2D<f64>,
}

impl Matrix {
    pub fn new(num_rows: usize, num_columns: usize) -> Matrix {
        return Matrix {
            grid: Array2D::filled_with(0.0, num_rows, num_columns)
        };
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&f64> {
        return self.grid.get(row, column);
    }

    pub fn set(&mut self, row: usize, column: usize, element: f64) ->  Result<(), Error> {
        return self.grid.set(row, column, element);
    }
}
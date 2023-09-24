use num::integer::gcd;

pub struct Matrix {
    /// Cells
    pub cells: Vec<Vec<i32>>,
    /// Number of rows
    pub rows_count: usize,
    /// Number of columns
    pub columns_count: usize,
}

impl Matrix {
    /// Matrix constructor.
    pub fn new(rows_count: usize, columns_count: usize) -> Self {
        Self {
            cells: vec![vec![0; columns_count]; rows_count],
            rows_count,
            columns_count,
        }
    }

    /// Adds two rows.
    pub fn add_rows(row_1: &[i32], row_2: &[i32]) -> Vec<i32> {
        debug_assert_eq!(row_1.len(), row_2.len(), "The rows are not of equal length.");

        row_1.iter().zip(row_2).map(|(x, y)| x + y).collect()
    }

    /// Returns the greatest common divisor of row.
    pub fn get_gcd_of_row(row: &[i32]) -> i32 {
        row.iter().fold(0, |result, &x| gcd(result, x))
    }

    /// Simplifies a row.
    pub fn simplify_row(row: &[i32]) -> Vec<i32> {
        let mut sign = 0;

        for integer in row {
            if *integer != 0 {
                sign = integer.signum();
                break;
            }
        }

        if sign == 0 {
            return row.to_owned()
        }

        let gcd_with_sign = sign * Self::get_gcd_of_row(row);

        row.iter().map(|&x| x / gcd_with_sign).collect()
    }

    /// Eliminates a matrix.
    pub fn eliminate(&mut self) {
        self.cells = self.cells.iter().map(|x| Matrix::simplify_row(x)).collect();

        let mut pivots_count = 0;

        for i in 0..self.columns_count {
            let mut pivot_row = pivots_count;

            while pivot_row < self.rows_count && self.cells[pivot_row][i] == 0 {
                pivot_row += 1;
            }

            if pivot_row == self.rows_count {
                continue;
            }

            let pivot = self.cells[pivot_row][i];

            if pivot_row != pivots_count {
                self.cells.swap(pivots_count, pivot_row);
            }

            pivots_count += 1;

            for j in pivots_count..self.rows_count {
                let gcd = gcd(pivot, self.cells[j][i]);
                self.cells[j] = Matrix::simplify_row(&Matrix::add_rows(
                    &self.cells[j].iter().map(|&x| pivot / gcd * x).collect::<Vec<i32>>(),
                    &self.cells[i].iter().map(|&x| -self.cells[j][i] / gcd * x).collect::<Vec<i32>>()
                ));
            }
        }

        for i in (0..self.rows_count).rev() {
            let mut pivot_column = 0;

            while pivot_column < self.columns_count && self.cells[i][pivot_column] == 0 {
                pivot_column += 1;
            }

            if i == 0 || self.columns_count == pivot_column {
                continue;
            }

            let pivot = self.cells[i][pivot_column];

            for j in (0..i).rev() {
                let gcd = gcd(pivot, self.cells[j][pivot_column]);
                self.cells[j] = Matrix::simplify_row(&Matrix::add_rows(
                    &self.cells[j].iter().map(|&x| pivot / gcd * x).collect::<Vec<i32>>(),
                    &self.cells[i].iter().map(|&x| -self.cells[j][pivot_column] / gcd * x).collect::<Vec<i32>>()
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::matrix::Matrix;

    /// Formats a matrix.
    pub fn format_matrix(matrix: &Matrix) -> String {
        let mut result = "[".to_string();

        for i in 0..matrix.rows_count {
            if i != 0 {
                result += ",\n";
            }

            result += &["[", &matrix.cells[i].iter().join(", "), "]"].join("");
        }

        result + "]"
    }

    #[test]
    fn test_new() {
        let matrix = Matrix::new(2, 3);
        assert_eq!(matrix.cells, vec![vec![0; 3]; 2]);
        assert_eq!(matrix.rows_count, 2);
        assert_eq!(matrix.columns_count, 3);
    }

    #[test]
    fn test_add_rows() {
        assert_eq!(Matrix::add_rows(&[1, 2, 3], &[4, 5, 6]), [5, 7, 9]);
    }

    #[test]
    fn test_get_gcd_of_row() {
        assert_eq!(Matrix::get_gcd_of_row(&[4, 8, 12]), 4);
    }

    #[test]
    fn test_simplify_row() {
        assert_eq!(Matrix::simplify_row(&[0, -2, 2, 4]), [0, 1, -1, -2]);
    }

    #[test]
    fn test_eliminate() {
        let mut matrix = Matrix::new(4, 4);
        matrix.cells = vec![
            vec![2, 0, -2, 0],
            vec![0, 2, -1, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        matrix.eliminate();
        assert_eq!(
            format_matrix(&matrix),
            "[[1, 0, -1, 0],\n[0, 2, -1, 0],\n[0, 0, 0, 0],\n[0, 0, 0, 0]]"
        );
    }
}

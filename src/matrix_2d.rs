use num_traits::identities::{self};
use std::ops::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix2D<
    T: Add<Output = T> + Mul<Output = T> + Copy + identities::One + identities::Zero,
    const ROWS: usize,
    const COLS: usize,
> {
    pub contents: [[T; COLS]; ROWS],
}

impl<
        T: Add<Output = T> + Copy + identities::One + identities::Zero,
        const ROWS: usize,
        const COLS: usize,
    > Add<Matrix2D<T, ROWS, COLS>> for Matrix2D<T, ROWS, COLS>
{
    type Output = Matrix2D<T, ROWS, COLS>;

    fn add(self, rhs: Matrix2D<T, ROWS, COLS>) -> Self::Output {
        let mut output_contents = [[T::zero(); COLS]; ROWS];

        for row_index in 0..ROWS {
            let mut current_row = [T::zero(); COLS];
            for column_index in 0..COLS {
                current_row[column_index] =
                    self.contents[row_index][column_index] + rhs.contents[row_index][column_index];
            }

            output_contents[row_index] = current_row;
        }

        Matrix2D {
            contents: output_contents,
        }
    }
}

impl<
        T: Add<Output = T> + Mul<Output = T> + Copy + identities::One + identities::Zero,
        const LHS_ROWS: usize,
        const SHARED_DIMENSION: usize,
        const RHS_COLS: usize,
    > Mul<Matrix2D<T, SHARED_DIMENSION, RHS_COLS>> for Matrix2D<T, LHS_ROWS, SHARED_DIMENSION>
{
    type Output = Matrix2D<T, LHS_ROWS, RHS_COLS>;

    fn mul(self, rhs: Matrix2D<T, SHARED_DIMENSION, RHS_COLS>) -> Self::Output {
        let mut output = Matrix2D {
            contents: [[T::zero(); RHS_COLS]; LHS_ROWS],
        };

        for column_index in 0..RHS_COLS {
            let rhs_column = rhs.get_column(column_index);
            let multiplied_column = self * rhs_column;
            output = output.set_column(multiplied_column, column_index);
        }

        output
    }
}

impl<
        T: Add<Output = T> + Mul<Output = T> + Copy + identities::One + identities::Zero,
        const ROWS: usize,
        const COLS: usize,
    > Mul<[T; COLS]> for Matrix2D<T, ROWS, COLS>
{
    type Output = [T; ROWS];

    fn mul(self, rhs: [T; COLS]) -> Self::Output {
        let mut output_value = [T::zero(); ROWS];

        for (row_index, current_row) in self.contents.iter().enumerate() {
            let mut row_value = T::zero();

            for (column_index, current_value) in current_row.iter().enumerate() {
                let scaled_value = *current_value * rhs[column_index];
                row_value = row_value + scaled_value;
            }

            output_value[row_index] = row_value;
        }

        output_value
    }
}

impl<
        T: Add<Output = T> + Mul<Output = T> + Copy + identities::One + identities::Zero,
        const ROWS: usize,
        const COLS: usize,
    > Matrix2D<T, ROWS, COLS>
{
    pub fn zero() -> Self {
        Matrix2D {
            contents: [[T::zero(); COLS]; ROWS],
        }
    }

    fn get_column(&self, column_index: usize) -> [T; ROWS] {
        let mut output = [T::zero(); ROWS];

        for row_index in 0..ROWS {
            output[row_index] = self.contents[row_index][column_index];
        }
        
        output
    }

    fn set_column(&self, new_value: [T; ROWS], column_index: usize) -> Self {
        let mut current_contents = self.contents;

        for row_index in 0..ROWS {
            current_contents[row_index][column_index] = new_value[row_index]
        }

        Matrix2D {
            contents: current_contents
        }
    }
}

impl<
        T: Add<Output = T> + Mul<Output = T> + Copy + identities::One + identities::Zero,
        const SIZE: usize,
    > Matrix2D<T, SIZE, SIZE>
{
    pub fn identity() -> Matrix2D<T, SIZE, SIZE> {
        let mut output_contents = [[T::zero(); SIZE]; SIZE];
        for pivot_index in 0..SIZE {
            output_contents[pivot_index][pivot_index] = T::one();
        }

        Matrix2D {
            contents: output_contents,
        }
    }
}

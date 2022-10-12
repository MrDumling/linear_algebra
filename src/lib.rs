mod matrix_2d;

#[cfg(test)]
mod tests {
    #[test]
    fn matrix_2d_addition() {
        use crate::matrix_2d::Matrix2D;

        let matrix_a = Matrix2D {
            contents: [[1, 2, 3], [4, 5, 6]],
        };

        let matrix_b = Matrix2D {
            contents: [[3, 6, 4], [1, 7, 2]],
        };

        let matrix_c = Matrix2D {
            contents: [[-3, 30, 42], [4, -15, 21]],
        };

        assert_eq!(matrix_a + matrix_b, matrix_a + matrix_b); // adding multiple times does not change the output: A + B = A + B
        assert_eq!(matrix_a + matrix_c, matrix_a + matrix_c);
        assert_eq!(matrix_b + matrix_c, matrix_b + matrix_c);

        assert_eq!(matrix_a + matrix_b, matrix_b + matrix_a); // commutative property: A + B = B + A
        assert_eq!(matrix_a + matrix_c, matrix_c + matrix_a);
        assert_eq!(matrix_b + matrix_c, matrix_c + matrix_b);

        assert_eq!(matrix_a + Matrix2D::zero(), matrix_a); // identity property of addition: A + 0 = A
        assert_eq!(matrix_b + Matrix2D::zero(), matrix_b);
        assert_eq!(matrix_c + Matrix2D::zero(), matrix_c);

        assert_eq!(
            matrix_a + (matrix_b + matrix_c),
            (matrix_a + matrix_b) + matrix_c
        ); //associative property: A + (B + C) = (A + B) + C

        let square_matrix = Matrix2D {
            contents: [[1, 2], [3, 4]],
        };

        assert_ne!(square_matrix + Matrix2D::identity(), square_matrix); // A + x != A, when x != 0
    }

    #[test]
    fn matrix_2d_multiplication() {
        use crate::matrix_2d::Matrix2D;

        let matrix_a = Matrix2D {
            contents: [[1, 2, 3], [4, 5, 6]],
        };

        let matrix_b = Matrix2D {
            contents: [[3, 6], [4, 5], [9, -1]],
        };

        assert_eq!(matrix_a * [3, 4, 5], [26, 62]);

        assert_eq!(
            Matrix2D {
                contents: [[-3, 42, -4, 50], [11, 34, 23, 14]]
            } * [-2, 15, -13, 4],
            [888, 245]
        );

        assert_eq!(
            matrix_a * matrix_b,
            Matrix2D {
                contents: [[38, 13], [86, 43]],
            }
        );
    }
}

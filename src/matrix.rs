use crate::equal_f32;
use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
    pub rows: [[f32; 2]; 2],
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
    pub rows: [[f32; 3]; 3],
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub rows: [[f32; 4]; 4],
}

impl Matrix2 {
    pub fn from_rows(rows: [[f32; 2]; 2]) -> Matrix2 {
        Matrix2 { rows }
    }

    pub fn determinant(&self) -> f32 {
        (self.rows[0][0] * self.rows[1][1]) - (self.rows[1][0] * self.rows[0][1])
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Matrix2) -> bool {
        for row in 0..2 {
            for col in 0..2 {
                if !equal_f32(self.rows[row][col], other.rows[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix3 {
    pub fn from_rows(rows: [[f32; 3]; 3]) -> Matrix3 {
        Matrix3 { rows }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        assert!(row < 3);
        assert!(col < 3);
        let mut values = Vec::with_capacity(4);
        for r in 0..3 {
            for c in 0..3 {
                if r == row || c == col {
                    continue;
                }
                values.push(self.rows[r][c]);
            }
        }
        Matrix2 {
            rows: [[values[0], values[1]], [values[2], values[3]]],
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for col in 0..3 {
            det += self.rows[0][col] * self.cofactor(0, col);
        }
        det
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Matrix3) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if !equal_f32(self.rows[row][col], other.rows[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix4 {
    pub fn from_rows(rows: [[f32; 4]; 4]) -> Matrix4 {
        Matrix4 { rows }
    }

    pub fn identity() -> Matrix4 {
        Matrix4::from_rows([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a new matrix where the rowss of the original matrix are
    /// turned into columns and the columns into rows.
    pub fn transpose(&self) -> Matrix4 {
        Matrix4::from_rows([
            [
                self.rows[0][0],
                self.rows[1][0],
                self.rows[2][0],
                self.rows[3][0],
            ],
            [
                self.rows[0][1],
                self.rows[1][1],
                self.rows[2][1],
                self.rows[3][1],
            ],
            [
                self.rows[0][2],
                self.rows[1][2],
                self.rows[2][2],
                self.rows[3][2],
            ],
            [
                self.rows[0][3],
                self.rows[1][3],
                self.rows[2][3],
                self.rows[3][3],
            ],
        ])
    }

    /// Returns a new matrix by removing the specified row and column.
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        assert!(row < 4);
        assert!(col < 4);
        let mut values = Vec::with_capacity(9);
        for r in 0..4 {
            for c in 0..4 {
                if r == row || c == col {
                    continue;
                }
                values.push(self.rows[r][c]);
            }
        }
        Matrix3 {
            rows: [
                [values[0], values[1], values[2]],
                [values[3], values[4], values[5]],
                [values[6], values[7], values[8]],
            ],
        }
    }

    /// Computes the determinant of the submatrix at the given row and column.
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 { minor } else { -minor }
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for col in 0..4 {
            det += self.rows[0][col] * self.cofactor(0, col);
        }
        det
    }

    pub fn is_invertible(&self) -> bool {
        !equal_f32(self.determinant(), 0.0)
    }

    /// Returns the inverse of the matrix, which reverses the effects
    /// of multiplying by the original matrix.
    pub fn inverse(&self) -> Matrix4 {
        let det = self.determinant();
        let mut values = Vec::with_capacity(16);
        for col in 0..4 {
            for row in 0..4 {
                let c = self.cofactor(row, col);
                values.push(c / det);
            }
        }
        Matrix4::from_rows([
            [values[0], values[1], values[2], values[3]],
            [values[4], values[5], values[6], values[7]],
            [values[8], values[9], values[10], values[11]],
            [values[12], values[13], values[14], values[15]],
        ])
    }

    /// Returns a transformation matrix that, when applied to a tuple,
    /// moves each component of the tuple by the given values.
    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4::from_rows([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a transformation matrix that, when applied to a tuple,
    /// scales each component of the tuple by the given values.
    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4::from_rows([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a transformation matrix that, when applied to a tuple,
    /// rotates the tuple around the x-axis.
    pub fn rotation_x(radians: f32) -> Matrix4 {
        Matrix4::from_rows([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, radians.cos(), -radians.sin(), 0.0],
            [0.0, radians.sin(), radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a transformation matrix that, when applied to a tuple,
    /// rotates the tuple around the y-axis.
    pub fn rotation_y(radians: f32) -> Matrix4 {
        Matrix4::from_rows([
            [radians.cos(), 0.0, radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-radians.sin(), 0.0, radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a transformation matrix that, when applied to a tuple,
    /// rotates the tuple around the z-axis.
    pub fn rotation_z(radians: f32) -> Matrix4 {
        Matrix4::from_rows([
            [radians.cos(), -radians.sin(), 0.0, 0.0],
            [radians.sin(), radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a transformation matrix that, when applied to a tuple,
    /// changes each component of the table in proportion to the other
    /// two components.
    pub fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix4 {
        Matrix4::from_rows([
            [1.0, x_y, x_z, 0.0],
            [y_x, 1.0, y_z, 0.0],
            [z_x, z_y, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a transformation matrix that orients the world relative
    /// to the camera. Specify the location of the camera (the `from`
    /// parameter), the point in the scene at which the camera should
    /// look (the `to` parameter), and a vector indicating which
    /// direction is up.
    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix4 {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);
        let orientation = Matrix4::from_rows([
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        orientation * Matrix4::translation(-from.x, -from.y, -from.z)
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !equal_f32(self.rows[row][col], other.rows[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut values = Vec::with_capacity(16);
        for row in 0..4 {
            for col in 0..4 {
                let mut value = 0.0;
                for i in 0..4 {
                    value += self.rows[row][i] * other.rows[i][col];
                }
                values.push(value);
            }
        }
        Matrix4::from_rows([
            [values[0], values[1], values[2], values[3]],
            [values[4], values[5], values[6], values[7]],
            [values[8], values[9], values[10], values[11]],
            [values[12], values[13], values[14], values[15]],
        ])
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        let x = (self.rows[0][0] * tuple.x)
            + (self.rows[0][1] * tuple.y)
            + (self.rows[0][2] * tuple.z)
            + (self.rows[0][3] * tuple.w);
        let y = (self.rows[1][0] * tuple.x)
            + (self.rows[1][1] * tuple.y)
            + (self.rows[1][2] * tuple.z)
            + (self.rows[1][3] * tuple.w);
        let z = (self.rows[2][0] * tuple.x)
            + (self.rows[2][1] * tuple.y)
            + (self.rows[2][2] * tuple.z)
            + (self.rows[2][3] * tuple.w);
        let w = (self.rows[3][0] * tuple.x)
            + (self.rows[3][1] * tuple.y)
            + (self.rows[3][2] * tuple.z)
            + (self.rows[3][3] * tuple.w);
        Tuple::new(x, y, z, w)
    }
}

#[cfg(test)]
mod tests {
    use crate::equal_f32;
    use crate::matrix::{Matrix2, Matrix3, Matrix4};
    use crate::tuple::Tuple;
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn test_constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m.rows[0][0], 1.0);
        assert_eq!(m.rows[0][3], 4.0);
        assert_eq!(m.rows[1][0], 5.5);
        assert_eq!(m.rows[1][2], 7.5);
        assert_eq!(m.rows[2][2], 11.0);
        assert_eq!(m.rows[3][0], 13.5);
        assert_eq!(m.rows[3][2], 15.5);
    }

    #[test]
    fn test_a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix2::from_rows([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(m.rows[0][0], -3.0);
        assert_eq!(m.rows[0][1], 5.0);
        assert_eq!(m.rows[1][0], 1.0);
        assert_eq!(m.rows[1][1], -2.0);
    }

    #[test]
    fn test_a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix3::from_rows([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(m.rows[0][0], -3.0);
        assert_eq!(m.rows[0][1], 5.0);
        assert_eq!(m.rows[0][2], 0.0);
        assert_eq!(m.rows[1][0], 1.0);
        assert_eq!(m.rows[1][1], -2.0);
        assert_eq!(m.rows[1][2], -7.0);
        assert_eq!(m.rows[2][0], 0.0);
        assert_eq!(m.rows[2][1], 1.0);
        assert_eq!(m.rows[2][2], 1.0);
    }

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let a = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_matrix_equality_with_different_matrices() {
        let a = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::from_rows([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }

    #[test]
    fn test_multiplying_two_matrices() {
        let a = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::from_rows([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert_eq!(
            a * b,
            Matrix4::from_rows([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn test_a_matrix_multiplied_by_a_tuple() {
        let a = Matrix4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix4::from_rows([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(a * Matrix4::identity(), a);
    }

    #[test]
    fn test_multiplying_the_identity_matrix_by_a_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(Matrix4::identity() * a, a);
    }

    #[test]
    fn test_transposing_a_matrix() {
        let a = Matrix4::from_rows([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        assert_eq!(
            a.transpose(),
            Matrix4::from_rows([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ])
        );
    }

    #[test]
    fn test_transposing_the_identity_matrix() {
        let a = Matrix4::identity().transpose();
        assert_eq!(a, Matrix4::identity());
    }

    #[test]
    fn test_calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix2::from_rows([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn test_a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix3::from_rows([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_eq!(
            a.submatrix(0, 2),
            Matrix2::from_rows([[-3.0, 2.0,], [0.0, 6.0]])
        )
    }

    #[test]
    fn test_a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = Matrix4::from_rows([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        assert_eq!(
            a.submatrix(2, 1),
            Matrix3::from_rows([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]),
        );
    }

    #[test]
    fn test_calculating_a_minor_of_a_3x3_matrix() {
        let a = Matrix3::from_rows([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn test_calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix3::from_rows([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix3::from_rows([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn test_calculating_the_determinant_of_a_4x4_matrix() {
        let a = Matrix4::from_rows([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn test_testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix4::from_rows([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn test_testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix4::from_rows([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn test_calculating_the_inverse_of_a_matrix() {
        let a = Matrix4::from_rows([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert!(equal_f32(b.rows[3][2], -160.0 / 532.0));
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert!(equal_f32(b.rows[2][3], 105.0 / 532.0));
        assert_eq!(
            b,
            Matrix4::from_rows([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn test_calculating_the_inverse_of_another_matrix() {
        let a = Matrix4::from_rows([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        assert_eq!(
            a.inverse(),
            Matrix4::from_rows([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn test_calculating_the_inverse_of_a_third_matrix() {
        let a = Matrix4::from_rows([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        assert_eq!(
            a.inverse(),
            Matrix4::from_rows([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
    }

    #[test]
    fn test_multiplying_a_product_by_its_inverse() {
        let a = Matrix4::from_rows([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix4::from_rows([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 4.0],
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse(), a);
    }

    #[test]
    fn test_multiplying_by_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix4::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(FRAC_PI_4);
        let full_quarter = Matrix4::rotation_x(FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
        );
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(FRAC_PI_4);
        let inv = half_quarter.inverse();
        assert_eq!(inv * p, Tuple::point(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn test_rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix4::rotation_y(FRAC_PI_4);
        let full_quarter = Matrix4::rotation_y(FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Tuple::point(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2)
        );
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_z(FRAC_PI_4);
        let full_quarter = Matrix4::rotation_z(FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Tuple::point(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(FRAC_PI_2);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);
        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn test_chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(FRAC_PI_2);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn test_the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);
        assert_eq!(t, Matrix4::identity());
    }

    #[test]
    fn test_a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);
        assert_eq!(t, Matrix4::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn test_the_view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);
        assert_eq!(t, Matrix4::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn test_an_arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);
        assert_eq!(
            t,
            Matrix4::from_rows([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000],
            ])
        );
    }
}

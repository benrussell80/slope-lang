pub trait FooBar {
    fn thing();
}

impl FooBar for () {
    fn thing() {
        ()
    }
}

fn foo_bar() -> impl FooBar {
    
}

use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R]
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(data: [[f64; C]; R]) -> Self {
        Self {
            data
        }
    }
}

impl<const R: usize, const C: usize, const Z: usize> Mul<Matrix<C, Z>> for Matrix<R, C> {
    type Output = Matrix<R, Z>;

    fn mul(self, rhs: Matrix<C, Z>) -> Self::Output {
        let mut data = [[0.0; Z]; R];

        for r in 0..R {
            for z in 0..Z {
                let mut sum = 0.0;
                for c in 0..C {
                    sum += self.data[r][c] * rhs.data[c][z];
                }
                data[r][z] = sum;
            }
        }

        Matrix {
            data
        }
    }
}


macro_rules! mat_mult {
    ($name:ident, $mat1:expr, $mat2:expr, $mat3:expr) => {
        #[test]
        fn $name() {
            let m1 = Matrix {
                data: $mat1
            };
            let m2 = Matrix {
                data: $mat2
            };
            assert_eq!(m1 * m2, Matrix { data: $mat3 })
        }
    };
}

mat_mult!(
    _2x2_2x2,
    [
        [1., 0.],
        [0., 1.]
    ],
    [
        [1., 2.],
        [3., 4.]
    ],
    [
        [1., 2.],
        [3., 4.]
    ]
);

mat_mult!(
    _2x2_2x1,
    [
        [1., 0.],
        [0., 1.]
    ],
    [
        [2.],
        [3.]
    ],
    [
        [2.],
        [3.]
    ]
);

mat_mult!(
    _3x3_3x1,
    [
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.]
    ],
    [
        [10.],
        [11.],
        [12.]
    ],
    [
        [10. + 22. + 36.],
        [40. + 55. + 72.],
        [70. + 88. + 108.]
    ]
);

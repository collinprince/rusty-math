macro_rules! vector {
    ( $name:ident<$T:ident>($inner:ty); ) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name<$T>($inner);
    };
}

vector! {
    Vector3<T>([T; 3]);
}

pub trait AdditiveIdentity {
    type Output;
    fn additive_identity() -> Self::Output;
}

pub trait MultiplicativeIdentity {
    type Output;
    fn multiplicative_identity() -> Self::Output;
}

impl AdditiveIdentity for usize {
    type Output = usize;
    fn additive_identity() -> Self::Output {
        0
    }
}

impl MultiplicativeIdentity for usize {
    type Output = usize;
    fn multiplicative_identity() -> Self::Output {
        1
    }
}

pub mod vector {
    pub trait VectorSpace {
        type Scalar;
        type Vector;
    }

    pub trait VAdd {
        type Vector;
        fn vadd(&self, l: &Self::Vector, r: &Self::Vector) -> Self::Vector;
    }

    pub trait VAddMut {
        type Vector;
        fn vadd_mut(&self, l: &mut Self::Vector, r: &Self::Vector);
    }

    pub trait VScale {
        type Scalar;
        type Vector;
        fn vscale(&self, vector: &Self::Vector, scalar: &Self::Scalar) -> Self::Vector;
    }

    pub trait VScaleMut {
        type Scalar;
        type Vector;
        fn vscale_mut(&self, vector: &mut Self::Vector, scalar: &Self::Scalar);
    }
}

use vector::*;

macro_rules! vector_space_inner {
    (@VScale $space:ident) => {
        impl VScale for $space {
            type Vector = <$space as VectorSpace>::Vector;
            type Scalar = <$space as VectorSpace>::Scalar;
            fn vscale(&self, vector: &Self::Vector, scalar: &Self::Scalar) -> Self::Vector {
                let mut buf = *vector;
                self.vscale_mut(&mut buf, scalar);
                buf
            }
        }
    };

    (@VScaleMut $space:ident) => {
        impl VScaleMut for $space {
            type Vector = <$space as VectorSpace>::Vector;
            type Scalar = <$space as VectorSpace>::Scalar;
            fn vscale_mut(&self, vector: &mut Self::Vector, scalar: &Self::Scalar) {
                use std::ops::MulAssign;
                vector.0.iter_mut().for_each(|val| val.mul_assign(scalar))
            }
        }
    };

    (@VAdd $space:ident) => {
        impl VAdd for $space {
            type Vector = <$space as VectorSpace>::Vector;
            fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector {
                let mut temp = *lhs;
                self.vadd_mut(&mut temp, rhs);
                temp
            }
        }
    };

    (@VAddMut $space:ident) => {
        impl VAddMut for $space {
            type Vector = <$space as VectorSpace>::Vector;
            fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector) {
                use std::ops::AddAssign;
                lhs.0
                    .iter_mut()
                    .zip(rhs.0)
                    .for_each(|(l, r)| l.add_assign(r))
            }
        }
    };
}

macro_rules! vector_space_expand {
    ( $($trait:ident, $space:ident),* ) => {
        $(
            vector_space_inner! { @$trait $space }
        )*
    };
}

macro_rules! vector_space {
    ($space:ident, $vector:ident, $scalar:ty) => {
        pub struct $space;
        impl VectorSpace for $space {
            type Scalar = $scalar;
            type Vector = $vector<$scalar>;
        }
        vector_space_expand! {
            VScaleMut, $space,
            VScale, $space,
            VAddMut, $space,
            VAdd, $space
        }
    };
}

// pub struct Matrix3<Vec>([Vec; 3]);

macro_rules! matrix {
    ( $name:ident<$T:ident>($inner:ty)  ) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name<$T>($inner);
    };
}

pub mod matrix_ops {
    pub trait MAdd {
        type Matrix;
        fn madd(&self, lhs: &Self::Matrix, rhs: &Self::Matrix) -> Self::Matrix;
    }

    pub trait MAddMut {
        type Matrix;
        fn madd_mut(&self, lhs: &mut Self::Matrix, rhs: &Self::Matrix);
    }
}

pub use matrix_ops::*;

impl MAdd for ThreeDimSpaceV2 {
    type Matrix = Matrix3X3<u32>;
    fn madd(&self, lhs: &Self::Matrix, rhs: &Self::Matrix) -> Self::Matrix {
        let mut temp = *lhs;
        self.madd_mut(&mut temp, &rhs);
        temp
    }
}

impl MAddMut for ThreeDimSpaceV2 {
    type Matrix = Matrix3X3<u32>;
    fn madd_mut(&self, lhs: &mut Self::Matrix, rhs: &Self::Matrix) {
        use std::ops::AddAssign;
        lhs.0.iter_mut().zip(rhs.0).for_each(|(l_row, r_row)| {
            l_row
                .0
                .iter_mut()
                .zip(r_row.0)
                .for_each(|(l, r)| l.add_assign(r))
        })
    }
}

matrix! {
    Matrix3X3<T>([Vector3<T>; 3])
}

vector_space! {
    ThreeDimSpaceV2, Vector3, u32
}

mod vec_tests {

    #[test]
    fn three_dim_space() {
        use crate::vector::*;
        use crate::{ThreeDimSpaceV2, Vector3};
        let space = ThreeDimSpaceV2;
        let u = Vector3([1u32, 2u32, 3u32]);
        let v = Vector3([3u32, 6u32, 9u32]);
        let result = space.vadd(&u, &v);
        let expected = Vector3([4u32, 8u32, 12u32]);
        assert_eq!(result, expected);
    }

    #[test]
    fn four_dim_space() {
        use crate::vector::*;
        // define Vector4
        vector! {
            Vector4<T>([T; 4]);
        }
        // define 4D space and operations
        vector_space! {
            FourDimSpace, Vector4, u32
        }

        let space = FourDimSpace;
        let u = Vector4([2u32, 4u32, 6u32, 8u32]);
        let v = Vector4([3u32, 6u32, 9u32, 12u32]);
        let result = space.vadd(&u, &v);
        let expected = Vector4([5, 10, 15, 20]);
        assert_eq!(result, expected);
    }

    #[test]
    fn matrix_add() {
        use crate::matrix_ops::*;
        use crate::{Matrix3X3, ThreeDimSpaceV2, Vector3};
        let space = ThreeDimSpaceV2;
        let x: Matrix3X3<u32> =
            Matrix3X3([Vector3([0, 1, 2]), Vector3([3, 4, 5]), Vector3([6, 7, 8])]);
        let y: Matrix3X3<u32> = Matrix3X3([
            Vector3([2, 4, 8]),
            Vector3([16, 32, 64]),
            Vector3([128, 256, 512]),
        ]);
        let result = space.madd(&x, &y);
        let expected: Matrix3X3<u32> = Matrix3X3([
            Vector3([2, 5, 10]),
            Vector3([19, 36, 69]),
            Vector3([134, 263, 520]),
        ]);
        println!("{:?}", result);
        assert_eq!(result, expected);
    }
}

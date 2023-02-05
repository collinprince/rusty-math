macro_rules! vector {
    ( $name:ident<$T:ident>($inner:ty); ) => {
        #[derive(Clone, Debug, PartialEq)]
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

macro_rules! vector_space_inner {
    (@VScale $space:ident) => {
        impl VScale for $space {
            type Vector = <$space as VectorSpace>::Vector;
            type Scalar = <$space as VectorSpace>::Scalar;
            fn vscale(&self, vector: &Self::Vector, scalar: &Self::Scalar) -> Self::Vector {
                let mut buf = vector.clone();
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
                let mut temp = lhs.clone();
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
    ($trait:ident, $space:ident) => {
        vector_space_inner! { @$trait $space }
    };
    ($trait:ident, $space:ident, $($trait_two:ident, $space_two:ident),+) => {
           vector_space_inner! { @$trait $space }
           vector_space_expand! { $($trait_two, $space_two),+ }
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
        impl VecOps for $space {}
    };
}

vector_space! {
    ThreeDimSpaceV2, Vector3, u32
}

pub trait VecOps: VAdd + VAddMut + VScale + VScaleMut {}

mod vec_tests {

    #[test]
    fn three_dim_space() {
        use crate::{ThreeDimSpaceV2, VAdd, Vector3};
        let space = ThreeDimSpaceV2;
        let u = Vector3([1u32, 2u32, 3u32]);
        let v = Vector3([3u32, 6u32, 9u32]);
        let result = space.vadd(&u, &v);
        let expected = Vector3([4u32, 8u32, 12u32]);
        assert_eq!(result, expected);
    }

    #[test]
    fn four_dim_space() {
        use crate::{VAdd, VAddMut, VScale, VScaleMut, VecOps, VectorSpace};
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
}

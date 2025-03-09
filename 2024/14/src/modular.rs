use num_traits::Euclid;

pub trait Modular
where
    Self: Sized,
{
    type Scalar: Euclid;
    const MODULUS: Self::Scalar;

    fn normalize<T>(n: T) -> Self::Scalar
    where
        T: From<Self::Scalar> + TryInto<Self::Scalar> + num_traits::Euclid,
    {
        n.rem_euclid(&T::from(Self::MODULUS))
            .try_into()
            .map_err(|_| panic!("impossible"))
            .unwrap()
    }
}

pub struct ParseModularError;

#[macro_export]
macro_rules! modular {
    ( $name:ident, $scalar:ty, $modulus:literal ) => {
        #[derive(Clone, Copy, Debug)]
        struct $name($scalar);

        impl modular::Modular for $name {
            type Scalar = $scalar;
            const MODULUS: $scalar = $modulus;
        }

        impl std::str::FromStr for $name {
            type Err = modular::ParseModularError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<i64>()
                    .map_err(|_| modular::ParseModularError)
                    .map(|s| s.into())
            }
        }

        impl<T> From<T> for $name
        where
            T: From<$scalar> + TryInto<$scalar> + num_traits::Euclid,
        {
            fn from(value: T) -> Self {
                Self(<Self as modular::Modular>::normalize(value))
            }
        }

        impl From<$name> for $scalar {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0.into()
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                *self = (self.0 + rhs.0).into();
            }
        }

        impl std::ops::Mul<$scalar> for $name {
            type Output = Self;

            fn mul(self, rhs: $scalar) -> Self::Output {
                (self.0 * rhs).into()
            }
        }
    };
}

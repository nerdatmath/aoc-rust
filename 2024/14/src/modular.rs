use num_traits::Euclid;
use parse_display::FromStr;

pub trait Modular
where
    Self: Sized,
{
    type Scalar: Euclid;
    const MODULUS: Self::Scalar;
}

#[derive(Clone, Copy, Debug, FromStr)]
#[from_str(new = <Self as From<isize>>::from(_0))]
pub struct Mod<const M: u8>(u8);

impl<const M: u8> Modular for Mod<M> {
    type Scalar = u8;
    const MODULUS: Self::Scalar = M;
}

impl<T, const M: u8> From<T> for Mod<M>
where
    T: From<isize> + From<u8> + TryInto<u8> + num_traits::Euclid,
{
    fn from(value: T) -> Self {
        Self(
            value
                .rem_euclid(&T::from(M))
                .try_into()
                .map_err(|_| panic!("impossible"))
                .unwrap(),
        )
    }
}

impl<const M: u8> From<Mod<M>> for u8 {
    fn from(value: Mod<M>) -> Self {
        value.0
    }
}

impl<const M: u8> From<Mod<M>> for usize {
    fn from(value: Mod<M>) -> Self {
        value.0.into()
    }
}

impl<const M: u8> std::ops::AddAssign for Mod<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 =
            u8::try_from((u16::from(self.0) + u16::from(rhs.0)).rem_euclid(u16::from(M))).unwrap();
    }
}

impl<const M: u8> std::ops::Neg for Mod<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(M - self.0)
    }
}

impl<const M: u8> std::ops::Mul<u8> for Mod<M> {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self(u8::try_from((u16::from(self.0) * u16::from(rhs)).rem_euclid(u16::from(M))).unwrap())
    }
}

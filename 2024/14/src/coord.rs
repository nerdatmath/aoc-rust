use std::{
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

use crate::orthant::Orthant;

#[derive(Clone, Copy, Debug)]
pub struct Coord<Row, Col> {
    pub row: Row,
    pub col: Col,
}

pub struct ParseCoordError;

impl<Row: FromStr, Col: FromStr> FromStr for Coord<Row, Col> {
    type Err = ParseCoordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col, row) = s.split_once(',').ok_or(ParseCoordError)?;
        Ok(Self {
            row: row.parse().map_err(|_| ParseCoordError)?,
            col: col.parse().map_err(|_| ParseCoordError)?,
        })
    }
}

macro_rules! coord_op_assign {
    ($trait:ident, $fn:ident) => {
        impl<Row: $trait, Col: $trait> $trait for Coord<Row, Col> {
            fn $fn(&mut self, rhs: Self) {
                self.row.$fn(rhs.row);
                self.col.$fn(rhs.col);
            }
        }
    };
}

macro_rules! coord_binop {
    ($trait:path, $fn:ident) => {
        impl<Row: $trait, Col: $trait> $trait for Coord<Row, Col> {
            type Output = Coord<<Row as $trait>::Output, <Col as $trait>::Output>;
            fn $fn(self, rhs: Self) -> Self::Output {
                Self::Output {
                    row: self.row.$fn(rhs.row),
                    col: self.col.$fn(rhs.col),
                }
            }
        }
    };
}

macro_rules! coord_op_scalar {
    ($ty:ident, $trait:path, $fn:ident) => {
        impl<$ty: Clone, Row: $trait, Col: $trait> $trait for Coord<Row, Col> {
            type Output = Coord<<Row as $trait>::Output, <Col as $trait>::Output>;
            fn $fn(self, rhs: $ty) -> Self::Output {
                Self::Output {
                    row: self.row.$fn(rhs.clone()),
                    col: self.col.$fn(rhs.clone()),
                }
            }
        }
    };
}

coord_op_assign!(AddAssign, add_assign);
coord_op_assign!(SubAssign, sub_assign);
coord_binop!(std::ops::Add, add);
coord_binop!(std::ops::Sub, sub);
coord_op_scalar!(T, std::ops::Mul<T>, mul);

impl<Row, Col> Orthant for Coord<Row, Col>
where
    Row: Orthant,
    Col: Orthant,
{
    type Output = (<Row as Orthant>::Output, <Col as Orthant>::Output);
    fn orthant(&self) -> Self::Output {
        (self.row.orthant(), self.col.orthant())
    }
}

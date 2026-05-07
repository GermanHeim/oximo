use std::ops::{Add, Mul, Neg, Sub};

use crate::handle::Expr;
use crate::linear::{add_into, mul_into, neg_into, sub_into};

// -----------------------------------------------------------------------------
// Expr <op> Expr
// -----------------------------------------------------------------------------

impl<'a> Add for Expr<'a> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let id = add_into(&mut self.arena.borrow_mut(), self.id, rhs.id);
        Self::new(id, self.arena)
    }
}

impl<'a> Sub for Expr<'a> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let id = sub_into(&mut self.arena.borrow_mut(), self.id, rhs.id);
        Self::new(id, self.arena)
    }
}

impl<'a> Mul for Expr<'a> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let id = mul_into(&mut self.arena.borrow_mut(), self.id, rhs.id);
        Self::new(id, self.arena)
    }
}

impl<'a> Neg for Expr<'a> {
    type Output = Self;
    fn neg(self) -> Self {
        let id = neg_into(&mut self.arena.borrow_mut(), self.id);
        Self::new(id, self.arena)
    }
}

// -----------------------------------------------------------------------------
// Expr <op> f64 / f64 <op> Expr, and the same for i32 because `2 * x`
// without type annotation is the most common ergonomic case.
// -----------------------------------------------------------------------------

macro_rules! impl_scalar_ops {
    ($scalar:ty) => {
        impl<'a> Add<$scalar> for Expr<'a> {
            type Output = Self;
            fn add(self, rhs: $scalar) -> Self {
                #[allow(clippy::cast_lossless)]
                let id = {
                    let mut a = self.arena.borrow_mut();
                    let rhs_id = a.constant(rhs as f64);
                    add_into(&mut a, self.id, rhs_id)
                };
                Self::new(id, self.arena)
            }
        }

        impl<'a> Add<Expr<'a>> for $scalar {
            type Output = Expr<'a>;
            fn add(self, rhs: Expr<'a>) -> Expr<'a> {
                rhs + self
            }
        }

        impl<'a> Sub<$scalar> for Expr<'a> {
            type Output = Self;
            fn sub(self, rhs: $scalar) -> Self {
                #[allow(clippy::cast_lossless)]
                let id = {
                    let mut a = self.arena.borrow_mut();
                    let rhs_id = a.constant(rhs as f64);
                    sub_into(&mut a, self.id, rhs_id)
                };
                Self::new(id, self.arena)
            }
        }

        impl<'a> Sub<Expr<'a>> for $scalar {
            type Output = Expr<'a>;
            fn sub(self, rhs: Expr<'a>) -> Expr<'a> {
                #[allow(clippy::cast_lossless)]
                let id = {
                    let mut a = rhs.arena.borrow_mut();
                    let lhs_id = a.constant(self as f64);
                    sub_into(&mut a, lhs_id, rhs.id)
                };
                Expr::new(id, rhs.arena)
            }
        }

        impl<'a> Mul<$scalar> for Expr<'a> {
            type Output = Self;
            fn mul(self, rhs: $scalar) -> Self {
                #[allow(clippy::cast_lossless)]
                let id = {
                    let mut a = self.arena.borrow_mut();
                    let rhs_id = a.constant(rhs as f64);
                    mul_into(&mut a, self.id, rhs_id)
                };
                Self::new(id, self.arena)
            }
        }

        impl<'a> Mul<Expr<'a>> for $scalar {
            type Output = Expr<'a>;
            fn mul(self, rhs: Expr<'a>) -> Expr<'a> {
                rhs * self
            }
        }
    };
}

impl_scalar_ops!(f64);
impl_scalar_ops!(i32);

// -----------------------------------------------------------------------------
// Sum support for `iter.sum::<Expr>()` would be nice but requires a starting
// value tied to the arena. Provide a free function instead.
// -----------------------------------------------------------------------------

/// Sum a non-empty iterator of expressions sharing the same arena.
///
/// # Panics
/// Panics if the iterator is empty.
pub fn sum<'a, I: IntoIterator<Item = Expr<'a>>>(iter: I) -> Expr<'a> {
    let mut it = iter.into_iter();
    let first = it.next().expect("oximo_expr::sum on empty iterator");
    it.fold(first, |acc, e| acc + e)
}

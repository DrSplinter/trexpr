pub mod expr;

use std::fmt::Display;

use futures::{Future, Stream};

use expr::Iter;
use expr::Lit;
use expr::Map;
use expr::Map2;
use expr::Map3;
use expr::Print;

//
// Expressions
//

pub trait Expr {
    type Item;
    type Stream: Stream<Item = Self::Item>;

    fn to_stream(self) -> Self::Stream;

    fn map<F, O>(self, f: F) -> Map<F, Self>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> O,
    {
        Map::new(f, self)
    }

    fn print(self) -> Print<Self>
    where
        Self: Sized,
        Self::Item: Display,
    {
        Print::new(self)
    }
}

//
// IntoExpression
//

pub trait IntoExpr {
    type Item;
    type IntoExpr: Expr<Item = Self::Item>;
    fn into_expr(self) -> Self::IntoExpr;
}

impl<E: Expr> IntoExpr for E {
    type Item = <Self as Expr>::Item;
    type IntoExpr = Self;

    fn into_expr(self) -> Self::IntoExpr {
        self
    }
}

impl IntoExpr for f32 {
    type Item = f32;

    type IntoExpr = Lit<f32>;

    fn into_expr(self) -> Self::IntoExpr {
        Lit::new(self)
    }
}

//
// Action
//

pub trait Action {
    type Item;
    type Future: Future<Output = ()>;

    fn to_future(self) -> Self::Future;
}

//
// Functions
//

pub fn when<C, T, O, OT>(
    cond: C,
    then: T,
    otherwise: O,
) -> Map3<fn(((C::Item, OT), OT)) -> OT, C::IntoExpr, T::IntoExpr, O::IntoExpr>
where
    C: IntoExpr<Item = bool>,
    T: IntoExpr<Item = OT>,
    O: IntoExpr<Item = OT>,
{
    Map3::new(
        |((cond, then), otherwise)| if cond { then } else { otherwise },
        cond.into_expr(),
        then.into_expr(),
        otherwise.into_expr(),
    )
}

pub fn lit<T>(val: T) -> Lit<T> {
    Lit::new(val)
}

pub fn iter<I>(iter: I) -> Iter<I>
where
    I: IntoIterator,
{
    Iter::new(iter)
}

pub fn map<F, T, TE1, TE2>((expr1, expr2): (TE1, TE2), f: F) -> Map2<F, TE1, TE2>
where
    TE1: Expr,
    TE2: Expr,
    F: FnMut((TE1::Item, TE2::Item)) -> T,
{
    Map2::new(f, expr1, expr2)
}

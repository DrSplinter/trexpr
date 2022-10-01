use futures::StreamExt;

use crate::Expr;

#[derive(Clone)]
pub struct Map2<F, A1, A2> {
    function: F,
    argument1: A1,
    argument2: A2,
}

impl<A1, A2, F, T> Map2<F, A1, A2>
where
    A1: Expr,
    A2: Expr,
    F: FnMut((A1::Item, A2::Item)) -> T,
{
    pub fn new(function: F, argument1: A1, argument2: A2) -> Self {
        Map2 {
            function,
            argument1,
            argument2,
        }
    }
}

impl<A1, A2, F, T> Expr for Map2<F, A1, A2>
where
    A1: Expr,
    A2: Expr,
    F: FnMut((A1::Item, A2::Item)) -> T,
{
    type Item = T;

    type Stream = futures::stream::Map<futures::stream::Zip<A1::Stream, A2::Stream>, F>;

    fn to_stream(self) -> Self::Stream {
        self.argument1
            .to_stream()
            .zip(self.argument2.to_stream())
            .map(self.function)
    }
}

use futures::StreamExt;

use crate::Expr;

#[derive(Clone)]
pub struct Map3<F, A1, A2, A3> {
    function: F,
    argument1: A1,
    argument2: A2,
    argument3: A3,
}

impl<A1, A2, A3, F, T> Map3<F, A1, A2, A3>
where
    A1: Expr,
    A2: Expr,
    A3: Expr,
    F: FnMut(((A1::Item, A2::Item), A3::Item)) -> T,
{
    pub fn new(function: F, argument1: A1, argument2: A2, argument3: A3) -> Self {
        Map3 {
            function,
            argument1,
            argument2,
            argument3,
        }
    }
}

impl<A1, A2, A3, F, T> Expr for Map3<F, A1, A2, A3>
where
    A1: Expr,
    A2: Expr,
    A3: Expr,
    F: FnMut(((A1::Item, A2::Item), A3::Item)) -> T,
{
    type Item = T;

    type Stream = futures::stream::Map<
        futures::stream::Zip<futures::stream::Zip<A1::Stream, A2::Stream>, A3::Stream>,
        F,
    >;

    fn to_stream(self) -> Self::Stream {
        self.argument1
            .to_stream()
            .zip(self.argument2.to_stream())
            .zip(self.argument3.to_stream())
            .map(self.function)
    }
}

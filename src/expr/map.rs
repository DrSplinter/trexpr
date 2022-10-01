use futures::StreamExt;

use crate::Expr;

#[derive(Clone)]
pub struct Map<F, A> {
    function: F,
    argument: A,
}

impl<A, F, T> Map<F, A>
where
    A: Expr,
    F: FnMut(A::Item) -> T,
{
    pub fn new(function: F, argument: A) -> Self {
        Map { function, argument }
    }
}

impl<A, F, T> Expr for Map<F, A>
where
    A: Expr,
    F: FnMut(<A::Stream as futures::Stream>::Item) -> T,
{
    type Item = T;

    type Stream = futures::stream::Map<A::Stream, F>;

    fn to_stream(self) -> Self::Stream {
        self.argument.to_stream().map(self.function)
    }
}

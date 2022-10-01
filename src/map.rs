use futures::{future, StreamExt};

use crate::Trexpr;

pub struct Map<F, A> {
    function: F,
    argument: A,
}

impl<A, F, T> Map<F, A>
where
    A: Trexpr,
    F: FnMut(A::Item) -> T,
{
    pub fn new(function: F, argument: A) -> Self {
        Map { function, argument }
    }
}

impl<A, F, T> Trexpr for Map<F, A>
where
    A: Trexpr,
    F: FnMut(<<A as Trexpr>::Stream as futures::stream::Stream>::Item) -> T,
{
    type Item = T;

    type Stream = futures::stream::Map<A::Stream, F>;

    fn to_stream(self) -> Self::Stream {
        self.argument.to_stream().map(self.function)
    }
}

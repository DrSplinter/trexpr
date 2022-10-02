use crate::Expr;

#[derive(Clone)]
pub struct Lit<T> {
    val: T,
}

impl<T> Lit<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T: Clone> Expr for Lit<T> {
    type Item = T;

    type Stream = futures::stream::Repeat<T>;

    fn to_stream(self) -> Self::Stream {
        futures::stream::repeat(self.val)
    }
}

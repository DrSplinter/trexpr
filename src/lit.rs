use crate::Trexpr;

pub struct Lit<T> {
    val: T,
}

impl<T> Lit<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T: Clone> Trexpr for Lit<T> {
    type Item = T;

    type Stream = futures::stream::Repeat<T>;

    fn to_stream(self) -> Self::Stream {
        futures::stream::repeat(self.val)
    }
}

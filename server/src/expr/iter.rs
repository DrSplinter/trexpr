use crate::Expr;

#[derive(Clone)]
pub struct Iter<T> {
    iter: T,
}

impl<T> Iter<T>
where
    T: IntoIterator,
{
    pub fn new(iter: T) -> Self {
        Iter { iter }
    }
}

impl<T> Expr for Iter<T>
where
    T: IntoIterator,
{
    type Item = T::Item;

    type Stream = futures::stream::Iter<T::IntoIter>;

    fn to_stream(self) -> Self::Stream {
        futures::stream::iter(self.iter)
    }
}

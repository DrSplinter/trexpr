use crate::{Action, Expr};
use futures::future::{ready, Ready};
use futures::StreamExt;
use std::fmt::Display;

pub struct Print<T> {
    arg: T,
}

impl<T> Print<T>
where
    T: Expr,
{
    pub fn new(arg: T) -> Self {
        Self { arg }
    }
}

impl<T> Action for Print<T>
where
    T: Expr,
    T::Item: Display,
{
    type Item = T::Item;
    type Future = futures::stream::ForEach<T::Stream, Ready<()>, fn(T::Item) -> Ready<()>>;

    fn execute(self) -> Self::Future {
        self.arg.to_stream().for_each(|x| {
            println!("{}", x);
            ready(())
        })
    }
}

pub mod lit;
pub mod map;

use tokio_stream::iter;
use futures::Stream;

use map::Map;

pub trait Trexpr {
    type Item;
    type Stream: Stream;

    fn to_stream(self) -> Self::Stream;


    fn map<F, O>(self, f: F) -> Map<F, Self>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> O,
    {
        Map::new(f, self)
    }
}

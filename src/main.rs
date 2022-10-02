use trexpr::expr::{Map, Map2};
use trexpr::{iter, when, Action, Expr};

#[derive(Clone)]
struct Bar {
    open: f32,
    close: f32,
}

impl Bar {
    fn new(open: f32, close: f32) -> Bar {
        Bar { open, close }
    }
}

//  Bar expression

trait BarExpr: Expr<Item = Bar> + Sized {
    fn close(self) -> Map<fn(Bar) -> f32, Self> {
        self.map(|bar| bar.close)
    }

    fn open(self) -> Map<fn(Bar) -> f32, Self> {
        self.map(|bar| bar.open)
    }
}

impl<E> BarExpr for E where E: Expr<Item = Bar> {}

//  Comparisson expression

trait CmpExpr: Expr + Sized {
    fn less_than<T>(self, other: T) -> Map2<fn((Self::Item, T::Item)) -> bool, Self, T>
    where
        T: Expr,
        Self::Item: PartialOrd<T::Item>,
    {
        trexpr::map((self, other), |(a, b)| a < b)
    }
}

impl<E> CmpExpr for E
where
    E: Expr,
    E::Item: PartialOrd,
{
}

#[tokio::main]
async fn main() {
    let prices = iter([
        Bar::new(1.0, 1.0),
        Bar::new(1.0, 1.6),
        Bar::new(1.7, 1.8),
        Bar::new(1.8, 1.5),
        Bar::new(1.4, 1.5),
    ]);

    let close = prices.clone().close();
    let open = prices.open();

    when(close.less_than(open), "be in NO trade", "be in BUY trade")
        .print()
        .execute()
        .await;
}

//
// Next level
//

// #[tokio::main]
// async fn main() {
//     let bf = bitfinex("eth", "btc", Bar1min);
//     let kr = kraken("eth", "btc", Bar1min);
//     let bf_got_above_kr = (&bf).close().got_above((&kr).close());
//     let bf_got_below_kr = bf.close().got_below(kr.close());
//     let time_bf_was_above_kr = time_since(&bf_got_above_kr, not(&bf_got_below_kr));

//     when(bf_got_below_kr.and(time_bf_was_above_kr.greate_than(100)), "Alert")
//         .send(alert_topic)
//         .send(dashboard)
//         .execute()
//         .await;
// }

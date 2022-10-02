use std::ops::Mul;

use trexpr::expr::{Map, Map2};
use trexpr::{iter, lit, when, Action, Expr};

#[derive(Clone)]
struct Candle {
    open: f64,
    close: f64,
}

impl Candle {
    fn new(open: f64, close: f64) -> Candle {
        Candle { open, close }
    }
}

//  Bar expression

trait BarExpr: Expr<Item = Candle> + Sized {
    fn close(self) -> Map<fn(Candle) -> f64, Self> {
        self.map(|bar| bar.close)
    }

    fn open(self) -> Map<fn(Candle) -> f64, Self> {
        self.map(|bar| bar.open)
    }
}

impl<E> BarExpr for E where E: Expr<Item = Candle> {}

//  Comparisson expression

trait CmpExpr: Expr + Sized {
    fn greater_than<T>(self, other: T) -> Map2<fn((Self::Item, T::Item)) -> bool, Self, T>
    where
        T: Expr,
        Self::Item: PartialOrd<T::Item>,
    {
        trexpr::map((self, other), |(a, b)| a > b)
    }
}

impl<E> CmpExpr for E
where
    E: Expr,
    E::Item: PartialOrd,
{
}

// Number expression

trait NumberExpr: Expr + Sized {
    fn scale<T, O>(self, other: T) -> Map2<fn((Self::Item, T::Item)) -> O, Self, T>
    where
        T: Expr,
        Self::Item: Mul<T::Item, Output = O>,
    {
        trexpr::map((self, other), |(a, b)| a * b)
    }
}

impl<E> NumberExpr for E
where
    E: Expr,
    E::Item: Mul,
{
}

#[tokio::main]
async fn main() {
    // let candles = bitfinex(Candle1min, "eth", "btc");
    let candles = iter([
        Candle::new(1.0, 1.0),
        Candle::new(1.0, 1.6),
        Candle::new(1.7, 1.8),
        Candle::new(1.8, 1.5),
        Candle::new(1.4, 1.5),
    ]);
    // let balance = balance(Bitfinex, api_key);
    let balance = iter([1000.0, 1000.0, 1000.0, 1000.0, 1000.0]);

    let close = candles.clone().close();
    let open = candles.open();

    when(close.greater_than(open), balance.scale(lit(0.01)), 0.0)
        .print()
        .execute()
        .await;
}

//
// Next level
//

// #[tokio::main]
// async fn main() {
//     let bf = bitfinex(Candle1min, "eth", "btc");
//     let kr = kraken(Candle1min, "eth", "btc");
//     let bf_got_above_kr = (&bf).close().got_above((&kr).close());
//     let bf_got_below_kr = bf.close().got_below(kr.close());
//     let time_bf_was_above_kr = time_since(&bf_got_above_kr, not(&bf_got_below_kr));

//     when(bf_got_below_kr.and(time_bf_was_above_kr.greate_than(100)), "Alert")
//         .send(alert_topic)
//         .send(dashboard)
//         .execute()
//         .await;
// }

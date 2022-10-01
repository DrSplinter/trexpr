use futures::StreamExt;
use trexpr::Trexpr;

#[tokio::main]
async fn main() {
    let x = trexpr::lit::Lit::new(10).map(|x| x + 1);
    let y: Vec<_> = x.to_stream().take(3).collect().await;

    println!("{:?}", y);
}

// fn main() {
//     //  let fst = File::from_str("numbers.txt").send(Email::new("j@j.com"));

//        let close = Binance::new("btc", "eth", Bar1Min).map(|ticker| ticker.close);
//        let ma = close.agg_n(avg, 5); // .ma(5)
//        let position = when(close.gt(ma), lit(1), lit(0)); // .if_then_else(1, 0)
//        let prev_position = position.shift(-1);
//        let signal = map2(|current, previous| current - previous, position, prev_position);
//        let buy = when(signal.eql(lit(1)), lit("BUY!"), empty());
//        let sell = when(signal.eql(lit(-1)), lit("SELL!"), empty());
//        let strategy = merge(buy, sell);

//        strategy.send(Console::new()).execute();
//    }

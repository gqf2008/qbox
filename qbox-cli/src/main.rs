mod app;
mod opt;
mod ui;
mod util;

use anyhow::Result;
use flexi_logger::{FileSpec, Logger};
use opt::Opt;
use qbox_core::broker::*;
use qbox_core::core::Event;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use url::Url;

fn main() -> Result<()> {
    let opt = Opt::from_file(std::env::args().nth(2).take().unwrap().as_str())?;
    let log_path = qbox_core::log_path();
    Logger::try_with_str(opt.level.as_str())?
        .format(flexi_logger::detailed_format)
        .log_to_file(
            FileSpec::default()
                .suppress_timestamp()
                .directory(Path::new(&log_path))
                .basename("qbox")
                .discriminant("qbox-cli")
                .suffix("log"),
        )
        .start()?;

    // qbox_core::core::startup()?;

    // qbox_broker::load_driver()?;

    // let trader = trader::spawn(Url::parse(opt.trade_dsn.as_str())?)?;
    // trader.instruments(&[]);
    // if let Some(instrs) = qbox_core::get_all_instrument() {
    //     let filter: Vec<_> = instrs
    //         .iter()
    //         .map(|instr| instr.security_id.clone())
    //         .collect();
    //     let filter: Vec<&str> = filter.iter().map(|sid| &**sid).collect();
    //     quoter.subscribe(&filter[..]);
    // } else {
    //     qbox_core::core::subscribe(topics::QUERY_EVENT, move |_topic, ev| {
    //         if let Event::Trade(TradeEvent::InstrumentsResponse(instr)) = ev.as_ref() {
    //             quoter.subscribe(&[instr.security_id.as_str()]);
    //         }
    //     })?;
    // }
    // // loop {
    // // }
    // qbox_core::core::subscribe(topics::QUOTES_EVENT, move |_, ev| {
    //     if let Event::Quote(QuoteEvent::Level1(instr)) = ev.as_ref() {}
    // });
    let mut app = app::App::new();
    app::run_app(&mut app)

    // loop {
    //     let rs = qbox_core::get_all_level1();
    //     println!("{:?}", rs);
    // }
    // Ok(())
}

pub mod assets;
pub mod db;
pub mod events;
pub mod orders;
pub mod positions;
pub mod qbox;
pub mod quotes;
pub mod topics;

pub use events::*;
pub use topics::*;

use anyhow::Result;

pub fn startup() -> Result<()> {
    //启动总线
    log::debug!("qbox events startup");
    broadcast(Event::Startup)?;
    log::debug!("qbox database startup");
    // //启动数据库
    // crate::db::startup()?;
    Ok(())
}

pub fn shutdown() -> Result<()> {
    broadcast(Event::Shutdown)
}

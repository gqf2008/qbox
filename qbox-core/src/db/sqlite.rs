use crate::broker::{Exchange, InstState, Instrument, Level1, TradeKind};
use crate::Parameter;
use anyhow::Result;
use rusqlite::{params, Connection, OpenFlags};
use std::path::Path;

const DB_NAME: &str = "qbox.db";
const SCHEMA: &str = include_str!("schema.sql");

pub(crate) fn init() -> Result<()> {
    init_table()?;
    Ok(())
}

fn init_table() -> Result<()> {
    let db = opendb()?;
    db.execute_batch(SCHEMA)?;
    let _ = db.close();
    Ok(())
}

pub(crate) fn create_bar_table(db: &Connection, name: &str) -> Result<()> {
    db.execute_batch(
        format!(
            "
        BEGIN;
        CREATE TABLE IF NOT EXISTS quote_bar_{} (
            security_id TEXT PRIMARY KEY,
            time INTEGER NOT NULL,
            avg REAL,
            open REAL,
            high REAL,
            low REAL,
            close REAL,
            last REAL,
            volume REAL,
            turnover REAL
        );
        COMMIT;
    ",
            name
        )
        .as_str(),
    )?;
    Ok(())
}

pub(crate) fn opendb() -> Result<Connection> {
    let db_path = Path::new(crate::data_path().as_str()).join(DB_NAME);
    let db = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
    )?;
    Ok(db)
}

pub fn opendb_read_only() -> Result<Connection> {
    let db_path = Path::new(crate::data_path().as_str()).join(DB_NAME);
    let db = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | { OpenFlags::SQLITE_OPEN_SHARED_CACHE },
    )?;
    Ok(db)
}

pub fn insert_instrument(db: &Connection, instr: &Instrument) -> Result<()> {
    const SQL: &str = r#"INSERT OR REPLACE INTO instruments (security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9);"#;
    let exchange: &str = instr.exchange.into();
    let kind: &str = instr.kind.into();
    let state = format!("{:?}", instr.state);
    let items = ron::to_string(&instr.items)?;
    db.execute(
        SQL,
        params![
            instr.security_id,
            exchange,
            instr.symbol,
            kind,
            instr.base_currency,
            instr.quote_currency,
            instr.multiplier,
            state,
            items
        ],
    )?;
    Ok(())
}

pub fn find_all_instruments(db: &Connection) -> Result<Vec<Instrument>> {
    let mut ret = vec![];
    const SQL:&str = "SELECT security_id,exchange,symbol,kind,base_currency,quote_currency,multiplier,state,items FROM instruments;";
    {
        let mut stat = db.prepare(SQL)?;
        let list = stat.query_map([], |row| {
            let items: String = row.get(8)?;
            let exchange: String = row.get(1)?;
            let kind: String = row.get(3)?;
            let state: String = row.get(7)?;
            let items: Parameter = if let Ok(items) = ron::from_str::<Parameter>(&items) {
                items
            } else {
                Parameter::new()
            };
            Ok(Instrument {
                security_id: row.get(0)?,
                exchange: Exchange::from(&exchange),
                symbol: row.get(2)?,
                kind: TradeKind::from(kind.as_str()),
                base_currency: row.get(4)?,
                quote_currency: row.get(5)?,
                multiplier: row.get(6)?,
                state: InstState::from(state.as_str()),
                items,
            })
        })?;
        for instr in list {
            ret.push(instr?);
        }
    }
    Ok(ret)
}

pub fn update_level1(db: &Connection, level1: &Level1) -> Result<()> {
    const SQL: &str = r#"INSERT OR REPLACE INTO quote_level1 (
        security_id,
        exchange,
        time,
        avg,
        open,
        high,
        low,
        close,
        last,
        last_volum,
        asks,
        bids,
        volume,
        turnover
    ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14);"#;
    let exchange: &str = level1.exchange.into();
    let asks = ron::to_string(&level1.asks)?;
    let bids = ron::to_string(&level1.bids)?;
    db.execute(
        SQL,
        params![
            level1.security_id,
            exchange,
            level1.time,
            level1.average,
            level1.open,
            level1.high,
            level1.low,
            level1.close,
            level1.last,
            level1.last_quantity,
            asks,
            bids,
            level1.volume,
            level1.turnover,
        ],
    )?;
    Ok(())
}

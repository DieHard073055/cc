use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum UnitError {
    #[error("Invalid Unit, please check help for correct list of units")]
    InvalidUnit,
}

#[derive(PartialEq)]
enum CryptoCurrency {
    Bitcoin,
    Ethereum,
    Solana,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Unit {
    BTC,
    MBTC,
    BIT,
    SATS,
    ETH,
    GWEI,
    WEI,
    FIN,
    SOL,
    LAM,
}

fn str_to_unit(value: &str) -> Result<Unit, UnitError> {
    match value.to_ascii_lowercase().as_str() {
        "btc" => Ok(Unit::BTC),
        "satoshi" => Ok(Unit::SATS),
        "mBTC" => Ok(Unit::MBTC),
        "bit" => Ok(Unit::BIT),
        "eth" => Ok(Unit::ETH),
        "gwei" => Ok(Unit::GWEI),
        "wei" => Ok(Unit::WEI),
        "finney" => Ok(Unit::FIN),
        "sol" => Ok(Unit::SOL),
        "lamports" => Ok(Unit::LAM),
        _ => Err(UnitError::InvalidUnit),
    }
}

impl TryFrom<&str> for Unit {
    type Error = UnitError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        str_to_unit(value)
    }
}

impl Unit {
    pub fn belongs_to_same_currency(a: &Unit, b: &Unit) -> bool {
        /*
        This function is used to check if 2 units belong to the same currency
         */
        let unit_curency_map: HashMap<Unit, CryptoCurrency> = HashMap::from([
            (Unit::BTC, CryptoCurrency::Bitcoin),
            (Unit::MBTC, CryptoCurrency::Bitcoin),
            (Unit::BIT, CryptoCurrency::Bitcoin),
            (Unit::SATS, CryptoCurrency::Bitcoin),
            (Unit::ETH, CryptoCurrency::Ethereum),
            (Unit::GWEI, CryptoCurrency::Ethereum),
            (Unit::FIN, CryptoCurrency::Ethereum),
            (Unit::WEI, CryptoCurrency::Ethereum),
            (Unit::SOL, CryptoCurrency::Solana),
            (Unit::LAM, CryptoCurrency::Solana),
        ]);

        unit_curency_map.get(&a).unwrap() == unit_curency_map.get(&b).unwrap()
    }
}

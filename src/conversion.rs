use crate::currency::{Unit, UnitError};
use rust_decimal::{Decimal, Error, dec};
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ConversionError {
    #[error("invalid amount, can only be numeric")]
    InvalidAmount(#[from] Error),
    #[error("Unable to convert amount, either too large or too small")]
    UnableToConvertAmount,
    #[error("invalid unit, please check help for valid units")]
    InvalidUnit(#[from] UnitError),
    #[error("incompatible units, please only convert units of the same currency")]
    IncompatibleUnits,
}
pub struct ConversionRequest {
    from_amount: Decimal,
    from_unit: Unit,
    to_unit: Unit,
}

impl ConversionRequest {
    pub fn new(from_amount: &str, from_unit: &str, to_unit: &str) -> Result<Self, ConversionError> {
        let (from_amount, from_unit, to_unit) =
            ConversionRequest::validate_conversion_request_params(from_amount, from_unit, to_unit)?;
        Ok(ConversionRequest {
            from_amount,
            from_unit,
            to_unit,
        })
    }
    fn validate_conversion_request_params(
        from_amount: &str,
        from_unit: &str,
        to_unit: &str,
    ) -> Result<(Decimal, Unit, Unit), ConversionError> {
        let from_amount = Decimal::from_str(from_amount)?;
        let from_unit = Unit::try_from(from_unit)?;
        let to_unit = Unit::try_from(to_unit)?;

        if !Unit::belongs_to_same_currency(&from_unit, &to_unit) {
            return Err(ConversionError::IncompatibleUnits);
        }

        Ok((from_amount, from_unit, to_unit))
    }
    pub fn convert(self) -> Result<Decimal, ConversionError> {
        /*
        We first convert amount to the smallest unit for a currency,
        then convert it to the target unit.
         */
        let unit_conversion_map: HashMap<Unit, Decimal> = HashMap::from([
            (Unit::BTC, dec!(100_000_000)),
            (Unit::MBTC, dec!(100_000)),
            (Unit::BIT, dec!(100)),
            (Unit::SATS, dec!(1)),
            (Unit::ETH, dec!(1_000_000_000_000_000_000)),
            (Unit::GWEI, dec!(1_000_000_000)),
            (Unit::FIN, dec!(1_000_000_000_000_000)),
            (Unit::WEI, dec!(1)),
            (Unit::SOL, dec!(1_000_000_000)),
            (Unit::LAM, dec!(1)),
        ]);

        let fa_mul = unit_conversion_map.get(&self.from_unit).unwrap().clone();
        let from_amount_smallest_denomination = self.from_amount.checked_mul(fa_mul);
        if from_amount_smallest_denomination.is_none() {
            return Err(ConversionError::UnableToConvertAmount);
        }
        let from_amount_smallest_denomination = from_amount_smallest_denomination.unwrap();
        let ta_div = unit_conversion_map.get(&self.to_unit).unwrap().clone();
        let to_amount_smallest_denomination = from_amount_smallest_denomination.checked_div(ta_div);
        if to_amount_smallest_denomination.is_none() {
            return Err(ConversionError::UnableToConvertAmount);
        }
        let to_amount_smallest_denomination = to_amount_smallest_denomination.unwrap();
        Ok(to_amount_smallest_denomination)
    }
}

#[cfg(test)]
mod test {
    use super::ConversionRequest;
    use rust_decimal::dec;

    #[test]
    fn test_convert_same_currency() {
        let c = ConversionRequest::new("1.5", "btc", "satoshi").unwrap();
        let r = c.convert().unwrap();
        assert_eq!(r, dec!(150_000_000));

        let c = ConversionRequest::new("21000000", "btc", "satoshi").unwrap();
        let r = c.convert().unwrap();
        assert_eq!(r, dec!(2_100_000_000_000_000));

        let c = ConversionRequest::new("1", "satoshi", "btc").unwrap();
        let r = c.convert().unwrap();
        assert_eq!(r, dec!(0.000_000_01));
    }
}

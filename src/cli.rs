use crate::conversion::{ConversionError, ConversionRequest};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CliErrors {
    #[error("no arguments were passed in")]
    NoArguments,
    #[error("invalid argument, please pass in <amount> <unit> to <unit>")]
    InvalidArgument,
    #[error("{0}")]
    ConversionError(#[from] ConversionError),
}

#[derive(Debug, PartialEq)]
enum CliMethods {
    HelpMessage,
    Convert,
}
fn conversion(args: Vec<String>) -> Result<(), CliErrors> {
    let fa = args[0].as_str();
    let fu = args[1].as_str();
    let tu = args[3].as_str();
    let cr = ConversionRequest::new(fa, fu, tu)?;
    let result = cr.convert()?;
    println!("{} {} = {} {}", fa, fu, result, tu);

    Ok(())
}
fn help() {
    println!("[usage]: <amount> <unit> to <unit>");
    println!("");
    println!("amount: numeric value, can only contain digits and '.' to indicate decimal place");
    println!("unit: can only contain units for a given cryptocurrency");
    println!("      possible values:");
    println!("          - Bitcoin:  btc, mbtc, bit, satoshi");
    println!("          - Ethereum: eth, finney, gwei, wei");
    println!("          - Solana:   sol, lamports");
    println!("");
    println!("example usage: 1.5 btc to satoshi");
    println!("");
}
fn _parse_arguments(args: &Vec<String>) -> Result<CliMethods, CliErrors> {
    match args.len() {
        0 => Err(CliErrors::NoArguments),
        1 => {
            if args[0].as_str() == "help" {
                Ok(CliMethods::HelpMessage)
            } else {
                Err(CliErrors::InvalidArgument)
            }
        }
        2..=3 => Err(CliErrors::InvalidArgument),
        4 => Ok(CliMethods::Convert),
        _ => Err(CliErrors::InvalidArgument),
    }
}
pub fn parse_arguments() -> Result<(), CliErrors> {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0); // remove the current application name
    let cli_method = _parse_arguments(&args)?;
    match cli_method {
        CliMethods::Convert => conversion(args)?,
        CliMethods::HelpMessage => help(),
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_args_select_convert() {
        let args = vec![
            "3400".to_string(),
            "BTC".to_string(),
            "to".to_string(),
            "satoshis".to_string(),
        ];
        let cli_method = _parse_arguments(&args).unwrap();
        assert_eq!(cli_method, CliMethods::Convert)
    }

    #[test]
    fn test_parse_args_select_help() {
        let args = vec!["help".to_string()];
        let cli_method = _parse_arguments(&args).unwrap();
        assert_eq!(cli_method, CliMethods::HelpMessage)
    }

    #[test]
    fn test_parse_args_empty() {
        let args = vec![];
        let cli_method = _parse_arguments(&args);
        assert!(cli_method.is_err());
        let err = cli_method.unwrap_err();
        assert_eq!(err, CliErrors::NoArguments);
    }

    #[test]
    fn test_parse_args_invalid() {
        let args = vec!["lol".to_string()];
        let cli_method = _parse_arguments(&args);
        assert!(cli_method.is_err());
        let err = cli_method.unwrap_err();
        assert_eq!(err, CliErrors::InvalidArgument);
    }
}

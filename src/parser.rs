use clap::Parser;
use num_bigint::BigUint;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Standard,
    Fermat,
    MillerRabin,
    Generate,
    Power,
    LucasLehmer,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The action to be performed on the target
    #[arg(short, long)]
    pub action: Action,

    /// The target number
    #[arg(short, long)]
    pub target: Option<BigUint>,

    /// The power to be raised to (Only used when analysis is `power`)
    #[arg(short, long)]
    pub power: Option<BigUint>,

    /// Number upto which primes to be generated (Only used when analysis is `generate`)
    #[arg(short, long)]
    pub maximum: Option<u64>,

    /// The exponent of mercenne prime for lucas lehmer test (Only used when analysis is `lucas-lehmer`)
    #[arg(short, long)]
    pub mersenne_exp: Option<BigUint>,
}

impl Args {
    pub fn get_action(&self) -> Action {
        self.action.clone()
    }

    pub fn get_target(&self) -> BigUint {
        let target = self.target.clone();
        match target {
            Some(target) => target,
            None => {
                println!("Use <exe> --help for more information (--target is required)");
                std::process::exit(1);
            }
        }
    }

    pub fn get_power(&self) -> BigUint {
        let power = self.power.clone();
        match power {
            Some(power) => power,
            None => {
                println!("Use <exe> --help for more information (--power is required)");
                std::process::exit(1);
            }
        }
    }

    pub fn get_maximum(&self) -> u64 {
        match self.maximum {
            Some(maximum) => maximum,
            None => {
                println!("Use <exe> --help for more information (--maximum is required)");
                std::process::exit(1);
            }
        }
    }

    pub fn get_mercenne_exp(&self) -> BigUint {
        let mercenne_power = self.mersenne_exp.clone();
        match mercenne_power {
            Some(mercenne_power) => mercenne_power,
            None => {
                println!("Use <exe> --help for more information (--mercenne_power is required)");
                std::process::exit(1);
            }
        }
    }
}

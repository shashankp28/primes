mod parser;

use clap::Parser;
use std::time::Instant;
use parser::Args;
use large_primes::get_max_primes;
use large_primes::{ standard, fermat, miller_rabin, lucas_lehmer_test };
use large_primes::pow;

fn main() {
    let args = Args::parse();

    let now = Instant::now();

    match args.get_action() {
        parser::Action::Power => {
            let target = args.get_target();
            let power = args.get_power();
            println!("Prime power {}: {}", target, pow(&target, &power));
        }
        parser::Action::Standard => {
            let target = args.get_target();
            let is_prime = standard(&target);
            println!("Standard Test: {} is prime: {}", target, is_prime);
        }
        parser::Action::Fermat => {
            let target = args.get_target();
            let is_prime = fermat(&target);
            println!("Fermat Test: {} is prime: {}", target, is_prime);
        }
        parser::Action::Generate => {
            let maximum = args.get_maximum();
            let primes = get_max_primes(maximum);
            println!("Primes upto {}: {:?}", maximum, primes);
        }
        parser::Action::MillerRabin => {
            let target = args.get_target();
            let is_prime = miller_rabin(&target);
            println!("Miller Rabin Test: {} is prime: {}", target, is_prime);
        }
        parser::Action::LucasLehmer => {
            let exp = args.get_mercenne_exp();
            let is_prime = lucas_lehmer_test(&exp);
            println!("Lucas Lehmer Test: M{} is prime: {}", exp, is_prime);
        }
    }
    let taken = now.elapsed();
    eprint!("Total time: {:?}", taken);
}

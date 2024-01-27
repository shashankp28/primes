# Prime Analysis Tool

Prime Analysis Tool is a versatile command-line utility written in Rust, designed to perform various mathematical operations and analyses on prime numbers and large integers. This tool offers a range of functionalities, including standard prime checks, advanced primality tests, number generation, exponentiation, and the Lucas-Lehmer test for Mersenne primes.

## Features

- **Standard**: Check if a number is prime using basic methods.
- **Fermat**: Perform the Fermat primality test.
- **Miller-Rabin**: Execute the Miller-Rabin primality test.
- **Generate**: Generate prime numbers up to a specified maximum.
- **Power**: Raise a number to a specified power.
- **Lucas-Lehmer**: Conduct the Lucas-Lehmer test for Mersenne primes.

## Installation

To install Prime Analysis Tool, ensure you have Rust and Cargo installed on your system. Follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/shashankp28/primes.git
   ```
2. Navigate to the project directory:
   ```
   cd primes
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Usage

To use Prime Analysis Tool, run the compiled binary with the required arguments. The general syntax is:

```
<exe> [OPTIONS] --action <ACTION> [--target <TARGET>] [--power <POWER>] [--maximum <MAXIMUM>] [--mersenne_exp <MERSENNE_EXP>]
```

### Options

- `-a`, `--action <ACTION>`: Specify the action to perform. Actions include `standard`, `fermat`, `miller-rabin`, `generate`, `power`, and `lucas-lehmer`.
- `-t`, `--target <TARGET>`: The target number for prime checks or exponentiation.
- `-p`, `--power <POWER>`: The power to raise the target number to (used with `power` action).
- `-m`, `--maximum <MAXIMUM>`: Specify the upper limit for prime number generation (used with `generate` action).
- `--mersenne-exp <MERSENNE_EXP>`: The exponent for the Mersenne prime in the Lucas-Lehmer test.

### Examples

- Get Help:

  ```
  ./target/release/primes --help
  ```

- Check if a number is a prime:
  ```
  ./target/release/primes --action standard --target 17
  ```
- Perform the Miller-Rabin primality test:
  ```
  ./target/release/primes --action miller-rabin --target 19
  ```
- Generate primes up to 100:
  ```
  ./target/release/primes --action generate --maximum 100
  ```
- Raise a number to a power:
  ```
  ./target/release/primes --action power --target 2 --power 10
  ```
- Perform the Lucas-Lehmer test:
  ```
  ./target/release/primes --action lucas-lehmer --mersenne-exp 13
  ```

## Contributing

Contributions to the Prime Analysis Tool are welcome! Feel free to submit issues or pull requests on GitHub.

## License

Prime Analysis Tool is distributed under the [MIT License](LICENSE).

## Author

[Shashank P](https://github.com/shashankp28)

---

Disclaimer: Prime Analysis Tool is a project under active development. Features and functionalities are subject to change.

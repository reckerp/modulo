# Modulo Calulator
This is a command-line application written in Rust that helps with calculations related to modulo calculations.
It currently supports three functionalities:

- Modulo Operation: Calculates the result of a modulo operation between two positive integers.
- Greatest Common Divisor (GCD): Calculates the GCD of two positive integers.
- Extended Euclidean Algorithm: Calculates the GCD of two integers and the coefficients of Bézout's identity.
- Modular Inverse: Calculates the modular inverse of an element (a) and a modulus (m).

## Usage
The application accepts commands in the following format:

```plaintext
mod [OPTION] <a> <modulus>

Options:
  -                  Calculate the modulo of a by b.
  --gcd              Calculate the GCD of two integers.
  --egcd             Calculate the GCD of two integers and the coefficients of Bézout's identity.
  --inv              Calculate the multiplicative modular inverse of an integer a (mod m).

```



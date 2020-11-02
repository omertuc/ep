fn main() {
    // find the primes up to this upper bound
    let sieve = slow_primes::Primes::sieve(1_000_000_000);

    fn sm(x: &usize) -> bool {
        let y = x.to_string();
        y.chars().nth(0).unwrap() == y.chars().last().unwrap()
    }

    for prime in sieve.primes().filter(|x| sm(x)) {
        println!("Hello {}", prime);
    }

}

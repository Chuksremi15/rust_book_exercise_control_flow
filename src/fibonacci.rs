pub fn fibonacci_recursive(n: u8) -> u8 {
    if n > 127 {
        panic!("n is too large");
    }

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
    }
}

pub fn fibonacci_loop(n: u8) -> u128 {
    if n > 100 {
        panic!("n is too large");
    }

    if n <= 1 {
        return n as u128;
    }

    let mut prev = 0u128;
    let mut current = 1u128;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}

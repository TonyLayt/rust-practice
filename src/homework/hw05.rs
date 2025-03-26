fn gcd(a: usize, b: usize) -> usize {

    if b == 0 {
        a
    } else {
        gcd(b, a % b) // Рекурсивный вызов
    }
}

pub fn test() {
    let data = [
        ((24, 60), 12),
        ((15, 9), 3),
        ((15, 6), 3),
        ((140, 40), 20),
        ((24, 16), 8),
        ((100, 10), 10),
        ((120, 80), 40),
        ((80, 120), 40),
        ((100, 20), 20),
        ((37, 11), 1),
        ((120, 90), 30),
    ];

    for ((a, b), exp) in data.iter() {
        println!("\nTesting GCD({}, {}):", a, b);
        let result = gcd(*a, *b);
        println!("Expected: {}, Got: {}\n", exp, result);
        assert_eq!(*exp, result);
    }
}


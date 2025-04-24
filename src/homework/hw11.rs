use rand::Rng;

// Генерує рандомний вектор довжиною `n` зі значеннями [10..99]
pub fn  gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Знаходить мінімальну суму сусідніх елементів та повертає їх як кортеж
fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_pair = (data[0], data[1]);
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
        }
    }

    Some(min_pair)
}

// Виводить вектор та мінімальну пару у зрозумілому вигляді
pub fn print_vector_and_min_pair(vec: &[i32]) {
    println!("Згенерований вектор: {:?}", vec);
    match min_adjacent_sum(vec) {
        Some((a, b)) => println!("Мінімальна пара з найменшою сумою: {} і {} (сума = {})", a, b, a + b),
        None => println!("Недостатньо елементів для пошуку пари"),
    }
}

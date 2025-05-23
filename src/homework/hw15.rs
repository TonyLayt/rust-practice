pub fn solve_muxa_slon() {
    let mut count = 0;

    for m in 1..9 { // від 1 до 8
        for u in 1..9 {
            if u == m { continue; }
            for x in 1..9 {
                if x == m || x == u { continue; }
                for a in 1..9 {
                    if a == m || a == u || a == x { continue; }

                    let muxa = m * 1000 + u * 100 + x * 10 + a;

                    let product = muxa * a;

                    // продукт має бути чотирицифровим
                    if product > 9999 { continue; }

                    let s = (product / 1000) % 10;
                    let l = (product / 100) % 10;
                    let o = (product / 10) % 10;
                    let n = product % 10;

                    // всі цифри мають бути від 1 до 8
                    let result_digits = [s, l, o, n];
                    if result_digits.iter().any(|&d| d < 1 || d > 8) {
                        continue;
                    }

                    // усі 8 цифр мають бути різними
                    let digits = [m, u, x, a, s, l, o, n];
                    let mut unique = digits.to_vec();
                    unique.sort();
                    unique.dedup();

                    if unique.len() == 8 {
                        println!("  {}{}{}{}", m, u, x, a);
                        println!("x      {}", a);
                        println!("  ------");
                        println!("  {}{}{}{}", s, l, o, n);
                        println!();
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Кількість рішень: {}", count);
}

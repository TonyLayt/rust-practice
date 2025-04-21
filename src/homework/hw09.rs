fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    // Зводимо зсув до меж рядка
    let mut n = n % len;
    if n < 0 {
        n += len; // перетворюємо від'ємний зсув на позитивний
    }

    // Розрізаємо рядок на 2 частини і міняємо місцями
    let split = (len - n) as usize;
    let (left, right) = s.split_at(split);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    for (n, expected) in shifts.iter() {
        // Отримуємо результат від rotate
        let result = rotate(s.clone(), *n);

        // Виводимо у консоль
        println!("Shift: {}, Result: {}, Expected: {}", n, result, expected);

        // Перевіряємо, чи результат відповідає очікуваному
        assert_eq!(result, expected.to_string());
    }
}

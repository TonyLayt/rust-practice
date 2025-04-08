fn invert_the_case(input: String) -> String {
    input.chars().map(|c| {

            if c.is_uppercase() {
                c.to_lowercase().to_string()

            } else if c.is_lowercase() {
                c.to_uppercase().to_string()

            } else {
                c.to_string()
            }
        })
        .collect()
}


pub fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];


    data.iter().for_each(|(a, b)| {
        // Виведення до перевірки
        println!("Перевірка до: a = '{}', b = '{}'", a, b);

        // Перевірка та виведення результату зміни регістру
        let result_a = invert_the_case(a.to_string());
        let result_b = invert_the_case(b.to_string());

        // Виведення результатів після зміни регістру
        println!("Результат після зміни для a: '{}'", result_a);
        println!("Результат після зміни для b: '{}'", result_b);

        // Перевірка на рівність
        assert_eq!(result_a, b.to_string());
        assert_eq!(result_b, a.to_string());

        // Виведення після перевірки
        println!("Перевірка після: a = '{}', b = '{}'", a, b);;
        });
}

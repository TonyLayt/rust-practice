pub fn draw_tree(triangle_count: usize) {
    // Проходимо по кожному трикутнику
    for i in 0..triangle_count {
        // Малюємо рядки для поточного трикутника
        for j in 0..=i {
            // Додаємо пробіли перед зірочками для вирівнювання
            let spaces = " ".repeat(triangle_count - j - 1);
            // Генеруємо рядок із зірочками
            let stars = "*".repeat(2 * j + 1);
            // Виводимо рядок у консоль
            println!("{}{}", spaces, stars);
        }
    }
}

#[test]
fn test (){
    draw_tree(4);
}
const HEIGHT: usize = 11; // Висота ромба (має бути непарною)
const WIDTH: usize = 11;  // Ширина ромба (має бути непарною)

pub fn drawing_a_diamond() {
    let mut output = String::new();
    let center = (HEIGHT / 2) as i32; // Центр ромба (індекс рядка/колонки)

    for i in 0..HEIGHT  {
        for j in 0..WIDTH {
            // Розраховуємо манхеттенську відстань від центру
            let distance = (i as i32 - center).abs() + (j as i32 - center).abs();
            if distance <= center {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n'); // Перехід на новий рядок після кожного рядка
    }

    print!("{}", output);  // Єдиний виклик print! для виведення всього тексту
}
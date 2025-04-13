const WIDTH: usize = 28;  // Ширина конверта (кількість символів)
const HEIGHT: usize = 10; // Висота конверта (кількість рядків)

pub fn draw_an_envelope() {
    let mut output = String::new(); // Буфер для виведення

    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            if y == 0 || y == HEIGHT {
                output.push('*'); // Верхня та нижня межі
            } else if x == 0 || x == WIDTH {
                output.push('*'); // Бокові межі
            } else if x == y* WIDTH / HEIGHT || x == WIDTH - y* WIDTH / HEIGHT {
                output.push('*'); // Діагональні лінії
            } else {
                output.push(' '); // Порожні місця всередині
            }
        }
        output.push('\n'); // Перехід на новий рядок
    }

    print!("{}", output); // Виводимо все за один виклик print!
}
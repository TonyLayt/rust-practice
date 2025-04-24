
pub fn count_permutation(shipments: &Vec<u32>) -> Result<usize, String> {
    let sum: u32 = shipments.iter().sum();
    let n = shipments.len();

    if sum as usize % n != 0 {
        return Err("Неможливо рівномірно розподілити вантаж".to_string());
    }

    let average = sum / n as u32;
    let mut moves = 0;

    for &val in shipments {
        if val > average {
            moves += (val - average) as usize;
        }
    }

    println!("Сума всіх чисел: {}", sum);
    println!("Середнє число: {}", average);
    println!("Мінімальна кількість переміщень: {}", moves);

    Ok(moves)
}

pub fn gen_shipments(mut shipments: Vec<u32>, target: u32) -> Vec<u32> {
    for i in 0..shipments.len() {
        if shipments[i] > target {
            shipments[i] = target;
        } else if shipments[i] < target {
            shipments[i] = target;
        }
    }

    println!("Усі числа прирівняні до середнього: {:?}", shipments);

    shipments
}
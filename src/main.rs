mod homework;

fn main() {
    println!("We learn Rust! \n\n");
    //homework::hw03::draw_an_envelope();
    //homework::hw04::drawing_a_diamond();
    //homework::hw05::test();
    //homework::hw06::draw_tree(6);
    //homework::hw07::test();
    //homework::hw08::prime_number(4);
    //homework::hw10::is_palindrome_number(1211);
    //let vec = homework::hw11::gen_random_vector(20);
    //homework::hw11::print_vector_and_min_pair(&vec);

    let arr = vec![8, 2, 2, 4, 4];
    match homework::hw12::count_permutation(&arr) {
        Ok(moves) => {
            let new_vec = homework::hw12::gen_shipments(arr.clone(), (arr.iter().sum::<u32>() / arr.len() as u32));
            println!("Новий вектор: {:?}", new_vec);
        },
        Err(e) => println!("Помилка: {}", e),
    }

}


pub fn prime_number (n: u32) {

    let mut bool_chek = true;

    for i in 2..n {
        println!("{}", i);
        if (n > 1 && n % i == 0){
            bool_chek = false;
            println!("{} false", n); // для відладки
        }
    }

    if bool_chek {
        println!("{} - просте число", n);
    }else {println!("{} - не просте число ", n);}
}
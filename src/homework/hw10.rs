pub fn is_palindrome_number(n: i32) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes(); // перетворюємо в масив байтів (символів)

    let mut left = 0;
    let mut right = bytes.len() - 1;

    while left < right {
        if bytes[left] != bytes[right] {
            println!("{} - не паліндром", n);
            return false; // якщо не співпали символи — це не паліндром
        }
        left += 1;
        right -= 1;
    }
    println!("{} - паліндром", n);
    true // якщо пройшли весь рядок — паліндром
}


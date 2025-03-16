use std::io;
fn is_prime(n: u32) -> bool {
    if n <= 1 {
    return false; 
    }
    for i in 2..=((n as f64).sqrt() as u32) {
    if n % i == 0 {
    return false;
    }
    }
    true
}
fn main() {
    //Зчитування числа
    println!("Введіть число:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося зчитати рядок");
    let num: u32 = input.trim().parse().expect("Будь ласка, введіть ціле число");
    //Перевірка чи є число простим
    if is_prime(num) {
    println!("Число {} є простим", num);
    } else {
    println!("Число {} не є простим", num);
    }
}

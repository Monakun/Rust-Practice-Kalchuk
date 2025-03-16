use std::io;
fn gcd(a: u32, b: u32) -> u32 {
  let mut x = a;
  let mut y = b;
  while y != 0 {
      let temp = y;
      y = x % y;
      x = temp;
    }
    x
}
fn main() {
    //Запитуємо користувача ввести числа
    println!("Введіть перше число:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Не вдалося прочитати рядок");
    let a: u32 = a.trim().parse().expect("Будь ласка, введіть коректне число");
    println!("Введіть друге число:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Не вдалося прочитати рядок");
    let b: u32 = b.trim().parse().expect("Будь ласка, введіть коректне число");
    let result = gcd(a, b);
    println!("Найбільший спільний дільник для {} та {}: {}", a, b, result);
}

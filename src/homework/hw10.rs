use std::io;
fn is_palindrome(num: i32) -> bool {
  let num_str = num.to_string();
  num_str == num_str.chars().rev().collect::<String>()
}
fn main() {
  println!("Введіть число:");
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Не вдалося прочитати");
  let number: i32 = input.trim().parse().expect("Введіть коректне число");
  if is_palindrome(number) {
      println!("Число {} є паліндромом!", number);
    } else {
      println!("Число {} не є паліндромом.", number);
    }
}

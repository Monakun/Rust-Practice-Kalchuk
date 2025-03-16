fn main() {
    const HEIGHT: usize = 13; //Висота
    const WIDTH: usize = 13;  //Ширина 
    let mut result = String::new();
    for i in 0..HEIGHT / 2 {
    let left_spaces = HEIGHT / 2 - i;
    let stars = 2 * i + 1; 
        result.push_str(&format!("{}{}\n", " ".repeat(left_spaces), "*".repeat(stars)));
    }
    result.push_str(&format!("{}\n", "*".repeat(WIDTH)));
    for i in (0..HEIGHT / 2).rev() {
    let left_spaces = HEIGHT / 2 - i; 
    let stars = 2 * i + 1;      
        result.push_str(&format!("{}{}\n", " ".repeat(left_spaces), "*".repeat(stars)));
    }
    //Вивід
    println!("{}", result);
}

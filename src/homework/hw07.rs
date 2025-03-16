fn toggle_case(input: &str) -> String {
input.chars()
    .map(|c| {
        if c.is_lowercase() {
            c.to_uppercase().to_string()
        } else if c.is_uppercase() {
            c.to_lowercase().to_string()
        } else {
            c.to_string()
        }
    })
         .collect()
}
fn main() {
let input = "HellO, WORld!";
let result = toggle_case(input);
println!("Оригінал: {}", input);
println!("Новий: {}", result);
}

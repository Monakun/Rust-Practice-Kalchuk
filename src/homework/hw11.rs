use std::time::{SystemTime, UNIX_EPOCH};
fn gen_random_vector(n: usize) -> Vec<i32> {
let mut rng = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
let mut vec = Vec::new();
for _ in 0..n {
    rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1); 
    let random_number = ((rng >> 30) & 0xFFFF) % 90 + 10; //Генеруємо число
    vec.push(random_number as i32);
    }
    vec
}
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
  let mut min_sum = i32::MAX;
  let mut min_index = (0, 0);
  for i in 0..data.len() - 1 {
  let sum = data[i] + data[i + 1];
  if sum < min_sum {
        min_sum = sum;
        min_index = (i, i + 1);
        }
    }
    (min_sum, min_index.0, min_index.1)
}
fn print_vector(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:3} ", i);
    }
    println!();
    print!("data:    ");
    for num in data {
        print!("{:3} ", num);
    }
    println!();
    let (min_sum, idx1, idx2) = min_adjacent_sum(data);
    println!(
        r"indexes:                    \__ __/                                                     
         min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1], data[idx2], min_sum, idx1, idx2
    );
}

fn main() {
let n = 20;
let data = gen_random_vector(n);

  print_vector(&data);
}

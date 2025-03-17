const WIDTH: usize = 32;  
const HEIGHT: usize = 16;  
fn main() {  
let mut lines = vec![String::new(); HEIGHT];  
for y in 0..HEIGHT {  
for x in 0..WIDTH {  
let c = if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {  
'*' 
} else if x == y * 2 || x == WIDTH - 1 - y * 2 {  
'*' 
} else {  
' ' 
};  
lines[y].push(c);  
}  
}  
print!("{}", lines.join("\n"));  
}

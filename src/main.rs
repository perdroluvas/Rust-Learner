use std::io;

fn main() {

    let mut A = String::new();
    let mut B = String::new();
    println!("Coloque seu primeiro numero!");
    io::stdin().read_line(&mut A).expect("Falhou");
    println!("Coloque seu segundo numero!");
    io::stdin().read_line(&mut B).expect("Falhou");

    let A = A.trim();
    let B = B.trim();

    //Parse?

    let A: i32 = match A.parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Coloque um inteiro valido!");
            return;
        }
    };

    let B: i32 = match B.parse(){
        Ok(num) => num,
        Err(_) =>{
            println!("Coloque um inteiro valido!");
            return;
        }
    };

    let result = divide(A, B);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

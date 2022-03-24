fn main() {
    //println!("Hello, world!");
    let a = 4;
    let b = 3;
    
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("Addition:{}", a + b);
    println!("Subtraction:{}", a - b);
    println!("Multiplication:{}", a * b);
    println!("Division:{}", a / b);
    println!("Modulus:{}", a % b);

    let mut c = 4;
    let mut d = 3;
    c = c + d; //7
    c = c * d; //21
    c = c - d; //18
    d = d - c; //-15
    println!("c:{}", c);
    println!("d:{}", d);
}

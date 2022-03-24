fn main() {
    //println!("Hello, world!");

    let mut a = 2;
    let mut b = 3;
    a += a; //4
    b -= b; //0
    a *= 1; //4
    b *= 3; //0
    a -= 1; //3
    println!("a: {}", a);
    println!("b: {}", b);
}

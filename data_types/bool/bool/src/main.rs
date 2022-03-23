fn main() {
    let test = true;
    let test2 = false;

    let result = test | test2;
    let newresult = test2 | test;
    let auto_check = test & test2;
    let value = 15 < 30;

    println!("{}",result);
    println!("{}",newresult);
    println!("{}",auto_check);
    println!("{}",value);
}

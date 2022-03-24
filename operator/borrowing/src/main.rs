fn main() {
    //println!("Hello, world!");

    // shared borrow
    let mut shared_numeric = 10;

    let mut copy = &shared_numeric;

    println!("{}",shared_numeric);
    println!("{}",copy);
    // can't change
    // copy = 100;

    let mut newcopy = &mut shared_numeric;
    println!("before {}",newcopy);
    // un commet the line below and check the error
    //println!("before {}",copy);
    *newcopy = 100;
    println!("after {}",newcopy);
    let _newcopy = &1000;
    println!("{}",_newcopy);

    //copy =1000;



    
}

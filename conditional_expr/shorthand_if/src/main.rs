// fn main() {
//     println!("Hello, world!");
// }
fn main() {
    //define a variable  
    let learn_language = "Just";
    // short hand construct
    let res= if learn_language == "Rust" {"You are learning Rust language!"} else {"You are learning some other language!"};
    println!("{}", res);

    let test_tuple = ("data",10,15);

    let status = if test_tuple.0 == "data" {"data is present"} else {"not available"};

    println!("{}",status);
}
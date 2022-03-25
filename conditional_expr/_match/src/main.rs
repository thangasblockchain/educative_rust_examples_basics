fn main() {
    //println!("Hello, world!");

    let mut name:&str = "thanga";
    let mut copy_name:&str = name.clone();
    copy_name = "new";
    name = copy_name;
    match name {
        "vidhya" => println!("vidhya"),
        "new" => println!("harshiv"),
        "thanga" => println!("thanga"),
        _ => println!("default")

    }
    name = "vidhya";
    let user = match name {
        "vidhya" => "vidhya",
        "new" => "hatshiv",
        "thanga" => "thanga",
        _ => "default"
    };
    println!("user name - {:?}",user);
}

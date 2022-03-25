fn main() {
    //println!("Hello, world!");
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("beginner", "Rust","course") = course {
        println!("case 1 Wrote all values in pattern to be matched with the scrutinee expression");
    }
    else {
        // do not execute this block
        println!("Value unmatched");
    }

    let course = ("Rust", "beginner","courses");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner", c) = course {
        println!("case 2 Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    } 
    else {
        // do not execute this block
        println!("Value unmatched");
    }

    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", c, d) = course {
        println!("case 3 Wrote one value in pattern to be matched with the scrutinee expression.Guessed values: {}, {}",c,d);
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    let course = ("Rust", "beginner");
    // pattern does not match with the scrutinee expression
    if let ("Java", c) = course {
        println!("Course is {}", c);
    } else {
        // execute this block
        println!("Value unmatched");
    }


}

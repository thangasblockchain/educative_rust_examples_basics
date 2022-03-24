// char in rust takes 4 bytes
// it can store lot more ASCII values like emoji,japanese,tamil

fn main() {
    //explicitly define a bool
    let _char = 'a';
    let _str = "string representation";
    let value = _str.len();
    let valu1 = _char.len_utf16();
    println!("{}",_char);
    println!("{}",_str);
    println!("{}",value);
    println!("{}",valu1);
}
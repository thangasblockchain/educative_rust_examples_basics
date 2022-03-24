#[allow(unused_variables, unused_mut)]
fn main() {
    //println!("Hello, world!");
    let arr = [1,2,3,4,5,6];
    let mut test:[i32;5] = [1,2,3,4,5];
    test = [1,2,4,5,6];
    test[3]=10;
    let testarr = arr;
    let slice_array:&[i32] = &arr[0..1];
    let person_data = ("a",'b',105,"300",'$');
    let (a,b,c,d,e) = person_data;
    let mut _person_data = ("Alex", 48, "35kg", "6ft");
    _person_data.0="John";
    println!("{:?}",test);
    println!("{} and {} {}",test[0],test[1],test[3]);
    println!("{:?}",testarr);
    println!("{:?}",slice_array);
    println!("{:?}",person_data);
    println!("{:?}",person_data.1);
    println!("{:?}{:?}{:?}{:?}{:?}",a,b,c,d,e);
    println!("{:?}",_person_data);

}

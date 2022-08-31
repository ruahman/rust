// 1. each value has an owner
// 2. there is only one owner at a time
// 3. when the owner goes out of scope the value disapears too

fn print_str(x: String) {
    println!("{}",x);
}
fn print_return_str(x: String) -> String {
    println!("{}",x);
    x
}
fn change_string(name: &mut String) {
    name.push_str("is happy");
    println!("{}",name);
}
pub fn demo(){
    let str1 = String::from("hello world");
    let str2 = str1;
    let str3 = str2.clone();
    // this causes problems
    // println!("{}", str1);
    println!("{}{}",str2,str3);
    // print_str(str1);

    // problem because it got moved to function
    // println!("{}",str1)
    let mut str4: String = String::from("test ");
    change_string(&mut str4);
    print_str(str4);

    let str5 = print_return_str(str3);
    print_return_str(str5);
    // let str4 = print_return_str(str1)
    

}

fn main() {
    let seb: &str = "Hello, Seb!";
    println!("{}", seb);

    let apples = 5;
    println!("I have {} apples.", apples);
    // apples = 10; // This line will cause a compile-time error because `apples` is immutable

    let mut change_me = 5;
    println!("Before change: {}", change_me);
    change_me = 10;
    println!("After change: {}", change_me);

    let mut my_name: String = String::new();
    my_name.push_str("Sebastien");
    println!("My name is: {}", my_name);

}

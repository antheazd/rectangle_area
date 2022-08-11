use std::io;

struct Rectangle{
    width: f32,
    length: f32,
}
//method that calculates rectangle surface
impl Rectangle{
    fn calculate_area(&self)-> f32 {
        self.length*self.width
}
}
fn main() {

    let mut string0 = String::new();
    println!("How many inputs do you want to have?");  
    io::stdin()
        .read_line(&mut string0)
        .expect("Invalid input");
    
    let number_of_iterations: u32 = string0.trim().parse().expect("Invalid input");
    
    for iterator in 0..number_of_iterations {

    let mut string1 = String::new();
    let mut string2 = String::new();

    println!("How wide is the rectange?");  
    io::stdin()
    .read_line(&mut string1)
    .expect("Invalid input");

    println!("How long is the rectange?");
    io::stdin()
    .read_line(&mut string2)
    .expect("Invalid input");

    let rectangle1 = Rectangle{
        width: string1.trim().parse().unwrap(),
        length: string2.trim().parse().unwrap(),
    };

    println!("Rectangle area equals {}", rectangle1.calculate_area());
}}



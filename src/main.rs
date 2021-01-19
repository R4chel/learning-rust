// chapter 3 exercises

fn fahrenheit_to_celcius(x:i32) -> i32 {
    (x-32)*5/9 
}

fn print_fahrenheit_and_celcius(x: i32) {
    println!("{} degrees fahrenheit is {} degrees celcius", x, fahrenheit_to_celcius(x));
}
fn main() {
    print_fahrenheit_and_celcius(0);
    print_fahrenheit_and_celcius(32);
    print_fahrenheit_and_celcius(-40);
}

// chapter 3 exercises

fn fahrenheit_to_celcius(x:i32) -> i32 {
    (x-32)*5/9 
}

fn print_fahrenheit_and_celcius(x: i32) {
    println!("{} degrees fahrenheit is {} degrees celcius", x, fahrenheit_to_celcius(x));
}

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        0
    }
    else if n == 1 {
        1
    }
    else {
        fibonacci(n-1) + fibonacci(n-2)
    }
} 
 
fn print_fibonacci(n: i32) {
    println!("The {}th fibonacci number is {}",n, fibonacci(n));
}
fn main() {
    print_fahrenheit_and_celcius(0);
    print_fahrenheit_and_celcius(32);
    print_fahrenheit_and_celcius(-40);
    print_fibonacci(0);
    print_fibonacci(1);
    print_fibonacci(2);
    print_fibonacci(3);
    print_fibonacci(4);
    print_fibonacci(10);
    print_fibonacci(-10);
}

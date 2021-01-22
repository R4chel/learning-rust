// chapter 3 exercises

fn fahrenheit_to_celcius(x: i32) -> i32 {
    (x - 32) * 5 / 9
}

fn print_fahrenheit_and_celcius(x: i32) {
    println!(
        "{} degrees fahrenheit is {} degrees celcius",
        x,
        fahrenheit_to_celcius(x)
    );
}

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn print_fibonacci(n: i32) {
    println!("The {}th fibonacci number is {}", n, fibonacci(n));
}

fn print_christmas_lyrics() {
    let presents = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a laying",
        "swans a singings",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for index in 0..(days.len()) {
        println!(
            "On the {} day of christmas my true love gave to me",
            days[index]
        );
        for present_index in (0..index + 1).rev() {
            let present_count: String = if present_index > 0 {
                (present_index + 1).to_string()
            } else if index == 0 {
                String::from("A")
            } else {
                String::from("And a")
            };
            println!(
                "{} {}{}",
                present_count,
                presents[present_index],
                if present_index == 0 { "." } else { "," }
            );
        }

        println!("")
    }
}
fn main() {
    print_christmas_lyrics();
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

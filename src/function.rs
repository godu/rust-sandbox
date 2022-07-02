pub fn main() -> () {
    println!("0°F is equal to {}°C", to_celcius(0.0));
    println!("0°C is equal to {}°F", to_fahrenheit(0.0));

    println!("The {}th Fibonnacci number is {}", 8, fibonacci(8));
}

fn to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn to_fahrenheit(celcius: f64) -> f64 {
    (celcius * 1.8) + 32.0
}

fn fibonacci(mut n: u8) -> u8 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut r = 0;
        let mut s = 1;

        while n > 2 {
            let t = r + s;
            r = s;
            s = t;
            n = n - 1
        }

        r + s
    }
}

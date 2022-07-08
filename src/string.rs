pub fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    let r1 = &s2;
    let r2 = &s2;
    println!("{}, {}", r1, r2);

    let s3 = String::from("hello world");
    let word = first_word(&s3[..]);
    println!("{}", word);

    next();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn next() {
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();
    let _s = String::from(data);

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}-{}-{}\n{}", s1, s2, s3, s);
}

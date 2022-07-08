pub fn main() {
    let _v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v__ = Vec::new();
    v__.push(5);
    v__.push(6);
    v__.push(7);
    v__.push(8);

    let _third: &i32 = &v[2];

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("The first element is: {}", first);
    v.push(6);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }
}

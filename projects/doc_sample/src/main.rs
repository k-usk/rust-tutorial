fn main() {
    let x = '1';
    let c = '2';

    match c {
        x => println!("x: {}", x),
//        _ => println!("other"),
    }

    println!("x: {}", x);
    println!("c: {}", c);
}

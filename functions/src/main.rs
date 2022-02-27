fn five() -> i32 {
    5
}

fn main() {
    let oi = ex() + 2;
    println!("{} {}", ex(), oi);

    let x = five();

    println!("The value of x is: {}", x);
    println!("{}", five())
}

fn ex() -> i32{
    let three = 3;
    return three
}
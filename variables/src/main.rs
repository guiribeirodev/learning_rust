fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let y = 20 / 6;
    let z = 20 % 6;

    println!("{} ,{}", y, z)

}
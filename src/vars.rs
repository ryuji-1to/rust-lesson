pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let _i1 = 3;
    let _f2 = 0.1;
    println!("{}", usize::BITS);
    println!("{:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("{:p}", &i2);
    println!("{:p}", &i3);

    let y = 5;
    println!("{:p}", &y);
    let y = y + 1;
    println!("{:p}", &y);
    let y = y * 2;
    println!("{:p}", &y);
    println!("{}", y);
    {
        let y = 0;
        println!("{}", y);
    }
    println!("{}", y);
    let t1 = (500, 6.4, "hello");
    let (x, y, z) = t1;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    println!("{}", t1.0);
    println!("{}", t1.1);
    println!("{}", t1.2);
}

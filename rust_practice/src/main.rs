fn main() {
    let pi = 3.141592;
    println!("Hello, world!");

    let s = format!("Pi is {a}  roughly {:.*}  ", 1, pi, a = "a");
    print!("{}\n", add(12, 45),);
    println!("{0}", s);
    println!("dasdf",);

    let _tup = (500, 6.4, 1);
    let (_x, y, _z) = _tup; //未使用变量加 _
    println!("{}", y);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

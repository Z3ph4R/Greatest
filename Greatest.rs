fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let num1 = 56;
    let num2 = 98;
    let result = gcd(num1, num2);
    println!("GCD of {} and {} is {}", num1, num2, result);
}

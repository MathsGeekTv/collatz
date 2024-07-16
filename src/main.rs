
fn main() {

    let mut n:u64 = 1_000_000;
    let start:u64 = n;
    let mut count_steps:u16 = 0;
    while n > 1 {
        n = collatz(n);
        count_steps += 1;
    }

    // assert_eq!(collatz(5), 16);

    println!("{} to 1 in {} steps", start,  count_steps);


}

fn collatz(n:u64) -> u64 {

    let x:u64;

    if (n % 2) == 0 {
        x = n / 2 ;
    } else {
        x = (3 * n) + 1;
    }
    return x;
}


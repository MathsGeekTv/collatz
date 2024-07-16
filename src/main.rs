use clap::Parser;

/// Simple program to output collatz sequence
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enter the number you wish to check
    #[arg(short, long)]
    number: u64,

    /// Show the steps in csv format
    #[arg(short, long)]
    steps: bool,

}

fn main() {

    let args = Args::parse();

    let mut n:u64 = args.number;
    let mut count_steps:u16 = 0;
    if args.steps {
        println!("Step,Number");
        println!("{},{}", n, count_steps);
    }
    while n > 1 {
        n = collatz(n);
        count_steps += 1;

        if args.steps {
            println!("{},{}", count_steps, n);
        }
    }

    // assert_eq!(collatz(5), 16);

    println!("{} to 1 in {} steps", args.number,  count_steps);


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


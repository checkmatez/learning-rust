use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut num = String::new();
    println!("Enter integer");
    std::io::stdin().read_line(&mut num).expect("fail");
    let num: u32 = num.trim().parse().expect("not a number");
    let now = Instant::now();
    println!("Fib for {} = {}", num, get_fib(num));
    println!("Duration = {}", now.elapsed().as_secs())
}

fn get_fib(n: u32) -> u128 {
    let mut memo: HashMap<u32, u128> = HashMap::new();

    fn fib(n: u32, memo: &mut HashMap<u32, u128>) -> u128 {
        if memo.contains_key(&n) {
            return memo[&n];
        }
        if n == 1 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        let result2 = fib(n - 2, memo);
        memo.insert(n - 2, result2);
        let result1 = fib(n - 1, memo);
        memo.insert(n - 1, result1);
        return result1 + result2;
    }

    fib(n, &mut memo)
}

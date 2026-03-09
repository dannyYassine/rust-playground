use std::fmt::Error;

#[path = "1.two-sum/solution.rs"]
mod two_sum;

#[path = "9.palindrome-number/solution.rs"]
mod palindrome;

#[path = "13.roman-to-integer/solution.rs"]
mod roman_to_integer;

#[path = "14.longest-common-prefix/solution.rs"]
mod longest_common_prefix;

#[tokio::main]
async fn main() -> Result<(), Error> {
    two_sum::Solution::run();
    palindrome::Solution::run();
    roman_to_integer::Solution::run();
    longest_common_prefix::Solution::run();
    Ok(())
}

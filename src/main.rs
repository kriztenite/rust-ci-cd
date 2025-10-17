/*run all functions in lib.rs */

use testrust::add;
use testrust::div;
use testrust::mul;
use testrust::sub;

fn main() {
    println!("Hello, world!");
    println!("add(1, 2) = {}", add(1, 2));
    println!("sub(1, 2) = {}", sub(1, 2));
    println!("mul(1, 2) = {}", mul(1, 2));
    println!("div(1, 2) = {}", div(1, 2));
}

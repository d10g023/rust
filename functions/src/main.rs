// function sum
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// function main
fn main() {
    // call function sum
    {
        let a = 1;
        let b = 2;
        let c = sum(a, b);
        println!("{} + {} = {}", a, b, c);
    }

    // use closure
    {
        let sum = |a: i32, b: i32| -> i32 { a + b };
        let a = 1;
        let b = 2;
        let c = sum(a, b);
        println!("{} + {} = {}", a, b, c);
    }

}
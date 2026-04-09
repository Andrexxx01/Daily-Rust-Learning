#![allow(unused_variables)] // This line allows us to have unused variables in our code without getting warnings. It's useful for learning purposes when we want to demonstrate concepts without necessarily using all variables.
const TAX_RATE: f64 = 7.25;
type Meters = i32;

fn main() {
    // 1. Learn main function and println! macro
    println!("Hello World ! I am a Rust Developer");

    // 2. Learn variables and basic arithmetic operations
    let apples = 5;
    let oranges = 7;
    let total_fruits = apples + oranges;
    let _differ_fruits = oranges - apples;
    println!("This year, my garden produced {apples} apples and {} oranges, totaling {} fruits!", oranges - 3, total_fruits);
    println!("This year, my garden produced {apples} apples and {oranges} oranges, totaling {total_fruits} fruits!");
    println!("This year, my garden produced {0} apples and {1} oranges, totaling {2} fruits! 
    I can't believe if I can produce {0} apples!", apples - 2, oranges, total_fruits);

    // 3. Learn mutability and variable reassignment
    let  mut gym_reps: i32 = 10;
    println!("I can do {gym_reps} push-ups in a row !");
    gym_reps = 15;
    println!("Now I can do {gym_reps} push-ups in a row !");

    // 4. Learn variable shadowing
    let grams_of_protein = "100.345";
    println!("The grams of protein I consume daily is {grams_of_protein} grams");
    let grams_of_protein = 100.345;
    println!("The grams of protein I consume daily is {grams_of_protein} grams");
    let mut grams_of_protein = 100;
    println!("The grams of protein I consume daily is {grams_of_protein} grams");
    grams_of_protein = 150;
    println!("The grams of protein I consume daily is {grams_of_protein} grams");

    // 5. Learn block scope and variable shadowing
    let coffee_price = 5.99;
    {
        let coffee_price = 4.99;
        println!("The price of coffee inside the block is ${coffee_price}");
    }
    println!("The price of coffee outside the block is ${coffee_price}");

    // 6. Learn constants
    println!("The tax rate is {}%", TAX_RATE);
    println!("The tax rate is {TAX_RATE}%");
    let income: i32 = 100000;
    println!("My income is ${income} and the tax I need to pay is ${:.2}", income as f64 * TAX_RATE / 100.0);

    // 7. Learn type aliases
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!("The length of a mile race is {mile_race_length} meters and the length of a two mile race is {two_mile_race_length} meters");

    // 8. Learn about unused variables and how to suppress warnings
    #[allow(unused_variables)]
    let marathon_length: Meters = 42195;
    #[allow(unused_variables)]
    let half_marathon_length: Meters = 21097;
    let mile_race_length: Meters = 1601;
    let two_mile_race_length: Meters = 3202;

    // 9. Learn about integer types and their ranges
    let eight_bit: i8 = -128;
    println!("The value of eight_bit is: {}", eight_bit);
    let eight_bit_unsigned: u8 = 255;
    println!("The value of eight_bit_unsigned is: {}", eight_bit_unsigned);
    let sixteen_bit: i16 = -32768;
    println!("The value of sixteen_bit is: {}", sixteen_bit);
    let sixteen_bit_unsigned: u16 = 65535;
    println!("The value of sixteen_bit_unsigned is: {}", sixteen_bit_unsigned);
    let thirty_two_bit: i32 = -2147483648;
    println!("The value of thirty_two_bit is: {}", thirty_two_bit);
    let thirty_two_bit_unsigned: u32 = 4294967295;
    println!("The value of thirty_two_bit_unsigned is: {}", thirty_two_bit_unsigned);
    let some_value = 20i8;
    println!("The value of some_value is: {}", some_value);
    let another_value = 30u16;
    println!("The value of another_value is: {}", another_value);
}

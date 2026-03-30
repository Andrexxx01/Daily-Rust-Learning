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
    let  mut gym_reps = 10;
    println!("I can do {gym_reps} push-ups in a row !");
    gym_reps = 15;
    println!("Now I can do {gym_reps} push-ups in a row !");
    // 4. Learn variable shadowing
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    let mut grams_of_protein = 100;
    grams_of_protein = 150;
    // 5. Learn block scope and variable shadowing
    let coffee_price = 5.99;
    {
        let coffee_price = 4.99;
        println!("The price of coffee inside the block is ${coffee_price}");
    }
    println!("The price of coffee outside the block is ${coffee_price}");
}

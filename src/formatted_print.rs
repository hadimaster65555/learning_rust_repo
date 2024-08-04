fn main() {
    // print with templating {}
    println!("My name is: {}", "Hadi");

    // print with positional argument
    println!("{0}, This is {1}. {1}, this is {0}", "Hadi", "Suryo");

    // use named argument
    println!(
        "{name}, use {tool} to do {task}", 
        name="Hadi",
        tool="Python",
        task="data analysis"
    );

    // use format
    println!("This is 100 in base 10: {}", 100);
    println!("This is 100 in base 2: {:b}", 100);
    println!("This is 100 in base 8: {:o}", 100);
    println!("This is 100 in base 16: {:x}", 100);   
}
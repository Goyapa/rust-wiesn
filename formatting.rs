// 1.2 Formatted print
fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}", 
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=7, width=3);

    println!("My name is {0}, {1}, {0}", "Bond", "James");

    let pi = 3.141592;
    let formatted_number = format!("{:.*}", 3, pi);
    println!("Pi is roughly {}", formatted_number);

    //  or short
    println!("Pi is roughly {:.*}", 4, pi);
}
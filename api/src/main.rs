fn main() {
    logging::setup_logging();
    let y = domain::add(44, 65);
    println!("The answer is: {y}");
}

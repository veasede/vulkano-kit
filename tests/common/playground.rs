use super::logger;


#[test]
fn playground_main() {
    logger::println("Normal");
    logger::eprintln("Error");
}
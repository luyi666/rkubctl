use clap::{load_yaml, App};
#[test]
fn test_yaml() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(i) = matches.value_of("middle") {
        println!("Value for input: {}", i);
    }

    if let Some(ref matches) = matches.subcommand_matches("image") {
        // "$ myapp image" was run
        if let Some(name) = matches.value_of("name") {
            println!("Value for name: {}", name);
        }
    }
}
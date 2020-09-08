#[macro_use]
extern crate clap;
use clap::App;
use std::fs;

fn main()  {
	let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let password = matches.value_of("password").unwrap_or("pass");
    println!("This is your password : {}", password);

    

    let metadata = fs::metadata("foo.txt").unwrap();
    println!("{}", metadata.len());

}
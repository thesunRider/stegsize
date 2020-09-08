#[macro_use]
extern crate clap;

use clap::App;
use std::fs;

fn main()  {
	let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("{:?}",matches );
    let password = matches.value_of("password").unwrap_or("pass");
    println!("This is your password : {}", password);

    let filer = matches.value_of("read").unwrap();
    println!("This is the file you wish to read : {}", filer);

    let metadata = fs::metadata(filer).unwrap();
    println!("{}", metadata.len());

}

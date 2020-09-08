#[macro_use]
extern crate clap;



use clap::App;
use std::fs;

fn main()  {
	let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    //println!("{:?}",matches.args );

	for (key, val) in &matches.args {
		let keynew = key;
    	match &keynew[..] {
    		"read" => read_file(&key,&val.vals[0].to_string_lossy(),&matches),
    		"write" => write_file(&key,&val.vals[0].to_string_lossy(),&matches),
    		_ => (),
    	}
	}
    

    let password = matches.value_of("password").unwrap_or("pass");
    println!("This is your password : {}", password);

    let filer = matches.value_of("read").unwrap();
    println!("This is the file you wish to read : {}", filer);

    let metadata = fs::metadata(filer).unwrap();
    println!("{}", metadata.len());

}

fn write_file(key:&str ,val:&str,matches:&clap::ArgMatches){
	println!("writing mode is active yo var1={var} var2={var2}",var=key,var2=val);
}

fn read_file(key:&str ,val:&str,matches: &clap::ArgMatches) {
	println!("matches are in read {:?}",matches.args );
	println!("reading shit var1={var} var2={var2}",var=key,var2=val);
}

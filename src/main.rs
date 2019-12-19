extern crate clap; 
use clap::{Arg, App};
use std::process::Command;

fn main() {
    // basic app information
    let app = App::new("mypipe program")
          .version("1.0")
          .about("Pipe command")
          .author("Pierre Sid-idris <pierre.sididris@live.fr>");

    // Command line option
    let in_option = Arg::with_name("in")
          .long("in")
          .takes_value(true)
          .help("String or fortune")
          .required(true);

     let out_option = Arg::with_name("out")
          .long("out")
          .takes_value(true)
          .help("Cowsay -f value")
          .required(true);

    // arguments to parse
    let app = app.arg(in_option).arg(out_option);

    // extract the matches
    let matches = app.get_matches();

     // Extract the actual name
     let in_opt = matches.value_of("in").expect("you must set a value");
     let out_opt = matches.value_of("out").expect("You can't set None value!");

     let input = if cfg!(target_os = "windows") {
     Command::new("cmd")
          .args(&["/C", "Not working on Windows, please run on linux"])
          .output()
          .expect("failed to execute process")
     } else {
          Command::new(in_opt)
               .output()
               .expect("failed to execute process")
     };
     let output = if cfg!(target_os = "windows") {
          Command::new("cmd")
               .args(&["/C", "Not working on Windows, please run on linux"])
               .output()
               .expect("failed to execute process")
          } else {
               Command::new(out_opt)
                    .arg(String::from_utf8(input.stdout).unwrap())
                    .output()
                    .expect("failed to execute process")
          };
     
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
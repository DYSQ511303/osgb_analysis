mod osgb;
use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new("OSGB Tool")
    .version("1.0")
    .about("OSGB file parser and converter")
    .subcommand(
        SubCommand::with_name("parse")
            .about("Parse OSGB file and output structure")
            .arg(
                Arg::with_name("INPUT")
                    .help("Input OSGB file path")
                    .required(true)
                    .index(1),
            ),
    )
    .subcommand(
        SubCommand::with_name("convert")
            .about("Convert OSGB to OSGT format")
            .arg(
                Arg::with_name("INPUT")
                    .help("Input OSGB file path")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("OUTPUT")
                    .help("Output OSGT file path")
                    .required(true)
                    .index(2),
            ),
    )
    .get_matches();

    match matches.subcommand() {
        ("parse", Some(parse_matches)) => {
            let input_path = parse_matches.value_of("INPUT").unwrap();
            
            match osgb::parse_osgb_file(input_path) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    process::exit(1);
                }
            }
        },
        ("convert", Some(convert_matches)) => {
            let input_path = convert_matches.value_of("INPUT").unwrap();
            let output_path = convert_matches.value_of("OUTPUT").unwrap();
            
            match osgb::convert_to_osgt(input_path, output_path) {
                Ok(_) => println!("Successfully converted to {}", output_path),
                Err(e) => {
                    eprintln!("Conversion error: {}", e);
                    process::exit(1);
                }
            }
        },
        _ => {
            eprintln!("No valid subcommand provided");
            process::exit(1);
        }
    }
}
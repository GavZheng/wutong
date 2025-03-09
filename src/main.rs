mod base;
mod base_conversion;
mod charcode;
mod color;
mod md5;

use clap::{Arg, ArgGroup, Command};
use colored::*;

fn main() {
    let matches = Command::new("Wutong")
        .version("0.1.0-alpha")
        .about("Wutong - A Swiss Army Knife of Developers.🌳")
        .author("Gavin Zheng<gav.zheng@outlook.com>")
        .subcommand(
            Command::new("bc")
                .about("Mathematical base conversion.")
                .arg(Arg::new("binary").long("bin").help("Binary number input."))
                .arg(Arg::new("octal").long("oct").help("Octal number input."))
                .arg(
                    Arg::new("decimal")
                        .long("dec")
                        .help("Decimal number input."),
                )
                .arg(
                    Arg::new("hexadecimal")
                        .long("hex")
                        .help("Hexadecimal number input."),
                )
                .group(
                    ArgGroup::new("base_conversion_options")
                        .args(["binary", "octal", "decimal", "hexadecimal"])
                        .required(true)
                        .multiple(false),
                ),
        )
        .subcommand(
            Command::new("base")
                .about("Base encoding and decoding.")
                .arg(
                    Arg::new("encode")
                        .short('e')
                        .long("encode")
                        .help("Encode the entered text."),
                )
                .arg(
                    Arg::new("decode")
                        .short('d')
                        .long("decode")
                        .help("Decode the entered text."),
                )
                .group(
                    ArgGroup::new("base_encoding_options")
                        .args(["encode", "decode"])
                        .required(true)
                        .multiple(false),
                ),
        )
        .subcommand(
            Command::new("md5").about("MD5 hashing.").arg(
                Arg::new("text")
                    .short('t')
                    .long("text")
                    .help("Input text to be hashed."),
            ),
        )
        .subcommand(
            Command::new("charcode")
                .about("Character code conversion.")
                .arg(
                    Arg::new("input")
                        .help("Parameters entered.")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("color").about("Color conversion.").arg(
                Arg::new("input")
                    .help("Parameters entered.")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("bc", subcommand_bc)) => match () {
            _ if subcommand_bc.contains_id("binary") => {
                let input = subcommand_bc.get_one::<String>("binary").unwrap();
                let result = base_conversion::math::binary(input);

                match result {
                    Ok(converted) => {
                        println!(
                            "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                            input, converted[0], converted[1], converted[2]
                        );
                    }
                    Err(error) => {
                        eprintln!("{}: {}", "Error converting binary".red(), error);
                        std::process::exit(1);
                    }
                }
            }
            _ if subcommand_bc.contains_id("octal") => {
                let input = subcommand_bc.get_one::<String>("octal").unwrap();
                let result = base_conversion::math::octal(input);

                match result {
                    Ok(converted) => {
                        println!(
                            "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                            converted[0], input, converted[1], converted[2]
                        );
                    }
                    Err(error) => {
                        eprintln!("{}: {}", "Error converting octal".red(), error);
                        std::process::exit(1);
                    }
                }
            }
            _ if subcommand_bc.contains_id("decimal") => {
                let input = subcommand_bc.get_one::<String>("decimal").unwrap();
                let result = base_conversion::math::decimal(input);

                match result {
                    Ok(converted) => {
                        println!(
                            "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                            converted[0], converted[1], input, converted[2]
                        );
                    }
                    Err(error) => {
                        eprintln!("{}: {}", "Error converting decimal".red(), error);
                        std::process::exit(1);
                    }
                }
            }
            _ if subcommand_bc.contains_id("hexadecimal") => {
                let input = subcommand_bc.get_one::<String>("hexadecimal").unwrap();
                let result = base_conversion::math::hexadecimal(input);

                match result {
                    Ok(converted) => {
                        println!(
                            "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                            converted[0], converted[1], converted[2], input
                        );
                    }
                    Err(error) => {
                        eprintln!("{}: {}", "Error converting hexadecimal".red(), error);
                        std::process::exit(1);
                    }
                }
            }
            _ => panic!("Invalid base conversion option"),
        },
        Some(("base", subcommand_base)) => match () {
            _ if subcommand_base.contains_id("encode") => {
                let input = subcommand_base.get_one::<String>("encode").unwrap();
                let result = base::text::encode::text_encode(input);

                match result {
                    Ok(converted) => println!("Base16: {}\nBase64: {}", converted[0], converted[1]),
                    Err(error) => {
                        eprintln!("{}: {}", "Error encoding".red(), error);
                        std::process::exit(1);
                    }
                }
            }
            _ if subcommand_base.contains_id("decode") => {
                let input = subcommand_base.get_one::<String>("decode").unwrap();
                let result = base::text::decode::text_decode(input);
                match result {
                    [Ok(converted_base16), Ok(converted_base64)] => {
                        println!("Base16: {}\nBase64: {}", converted_base16, converted_base64);
                    }
                    [Err(error), Ok(converted)] => {
                        eprintln!("{} {}", "Base16 decode error:".red(), error);
                        println!("Base64: {}", converted);
                    }
                    [Ok(converted), Err(error)] => {
                        println!("Base16: {}", converted);
                        eprintln!("{}: {}", "Base64 decode error".red(), error);
                    }
                    [Err(error_base16), Err(error_base64)] => {
                        eprintln!("{} {}", "Base16 decode error:".red(), error_base16);
                        eprintln!("{} {}", "Base64 decode error:".red(), error_base64);
                    }
                }
            }
            _ => panic!("{}", "Error: Invalid base conversion option".red()),
        },
        Some(("md5", subcommand_md5)) => {
            let result = md5::text::md5_text(subcommand_md5.get_one::<String>("text").unwrap());
            println!("{}", result);
        }
        Some(("charcode", subcommand_charcode)) => {
            let result = charcode::ascii_unicode::ascii_unicode(
                subcommand_charcode.get_one::<String>("input").unwrap(),
            );
            match result {
                Ok(result) => println!("ASCII: {}\nUnicode: {}", result[0], result[1]),
                Err(error) => println!("{} {}", "Error:".red(), error),
            }
        }
        Some(("color", subcommand_color)) => {
            let result = color::conversion::color_conversion(
                subcommand_color.get_one::<String>("input").unwrap(),
            );
            match result {
                Ok(result) => println!("RGB: {}\nHex: {}", result.0, result.1),
                Err(error) => println!("{} {}", "Error:".red(), error),
            }
        }
        _ => {}
    }
}

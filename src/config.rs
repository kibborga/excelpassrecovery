pub mod config {

    use clap::{arg, Command};
    use std::collections::HashMap;

    use crate::error;
    use std::clone::Clone;

    #[derive(Clone)]
    pub struct ConfigOptions {
        pub src_file: String,
        pub min: u8,
        pub max: u8,
        pub prefix: String,
        pub suffix: String,
        pub capitalize: bool,
        pub charset: String,
        pub threads: u8,
        pub verbose: bool,
    }

    lazy_static! {

        /**
         * Parse the command line options
         */
        pub static ref OPTS: HashMap<&'static str, String> = {
            let args = parse_cmdline_args().get_matches();
            let mut opts = HashMap::new();

            if let Some(c) = args.get_one::<String>("INPUT") {
                opts.insert("src_file", c.to_string());
            }

            if let Some(c) = args.get_one::<bool>("verbose") {
                opts.insert("verbose", if *c {"true".to_string()} else {"false".to_string()});
            }

            if let Some(c) = args.get_one::<String>("min") {
                opts.insert("min", c.to_string());
            }

            if let Some(c) = args.get_one::<String>("max") {
                opts.insert("max", c.to_string());
            }

            if let Some(c) = args.get_one::<String>("charset") {
                opts.insert("charset", c.to_string());
            }

            if let Some(c) = args.get_one::<String>("threads") {
                opts.insert("threads", c.to_string());
            }

            if let Some(c) = args.get_one::<String>("prefix") {
                opts.insert("prefix", c.to_string());
            }

            if let Some(c) = args.get_one::<String>("suffix") {
                opts.insert("suffix", c.to_string());
            }

            if let Some(c) = args.get_one::<bool>("capitalize") {
                opts.insert("capitalize", if *c {"true".to_string()} else {"false".to_string()});
            }

            opts
        };

    }

    /**
     * Parses the command line arguments
     */
    fn parse_cmdline_args() -> Command {
        return Command::new("ktxtool")
            .propagate_version(true)
            .args(&[
                arg!(<INPUT> "the input file name"),
                arg!(-m --min <MIN> "minimum password length (default: 1)"),
                arg!(-M --max <MAX> "maximum password length (default: 10)"),
                arg!(-c --charset <CHARSET> "charset to use (default: <a..z,0..9>)"),
                arg!(-t --threads <THREADS> "number of threads to use (default: 1)"),
                arg!(-p --prefix <PREFIX> "prefix to use"),
                arg!(-s --suffix <SUFFIX> "suffix to use"),
                arg!(-C --capitalize "capitalize first letter"),
                arg!(-v --verbose "verbose mode"),
            ])
            .version("0.1.0")
            .author(
                "Chavo"
            )
            .about("Excel password cracker. Supports .xls and .xlsx formats.");
    }

    /**
     * Returns the parsed command line options
     */
    pub fn get() -> ConfigOptions {

        let ext = std::path::Path::new(&option("src_file", ""))
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_lowercase();

        if ext != "xls" && ext != "xlsx" {
            error!("Invalid input file format");
            std::process::exit(1);
        }

        let opts = ConfigOptions {
            src_file: option("src_file", ""),
            min: option("min", "1").parse::<u8>().unwrap(),
            max: option("max", "10").parse::<u8>().unwrap(),
            charset: option("charset", "abcdefghijklmnopqrstuvwxyz0123456789"),
            threads: option("threads", "1").parse::<u8>().unwrap(),
            verbose: option("verbose", "false") == "true",
            prefix: option("prefix", ""),
            suffix: option("suffix", ""),
            capitalize: option("capitalize", "false") == "true",
        };

        opts

    }

    /**
     * Returns the value of the specified option
     */
    pub fn option(name: &str, default: &str) -> String {
        return OPTS.get(name).unwrap_or(&String::from(default)).to_string();
    }


}

pub use config::*;

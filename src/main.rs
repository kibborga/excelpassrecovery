// https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/multthreading.html
extern crate load_file;

use itertools::Itertools;
use office_crypto::decrypt_from_bytes;

use std::io::Cursor;
use std::path::Path;
use std::thread;

#[macro_use]
extern crate lazy_static;
pub mod config;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        if  config::option("verbose", "false") == "true" {
            println!($($arg)*);
        }
    })

}
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ({
        eprint!("! Warning: ");
        eprint!($($arg)*);
        eprintln!();
    })
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        eprint!("* Error: ");
        eprint!($($arg)*);
        eprintln!();
    })
}

fn main() {
    // parse command line arguments and get the options
    let opts = config::get();

    let path = Path::new(&opts.src_file);

    // check if file exists
    if !path.exists() {
        error!("File not found: {}", opts.src_file);
        std::process::exit(1);
    }

    let bytes = load_file::load_file_bytes(&path);
    if bytes.is_err() {
        error!("Error reading file: {}", opts.src_file);
        std::process::exit(1);
    }
    let bytes_vector = Vec::<u8>::from(bytes.unwrap());

    let chars = opts.charset.clone();

    // convert to tuple
    let mut cnt = 0;
    let mut total = 0;

    // start a timer
    let now = std::time::Instant::now();

    for pass_len in opts.min..opts.max + 1 {
        let char_tuple = chars.chars().unique();
        let perms = char_tuple.combinations_with_replacement(pass_len as usize);
        let prefix = opts.prefix.clone();
        let suffix = opts.suffix.clone();

        println!(
            "Trying password length {}",
            pass_len
        );

        let mut attempts = Vec::<String>::new();

        perms.for_each(|perm| {
            let t = opts.threads;
            let variation = perm.iter().collect::<String>();
            let mut password = prefix.clone();

            if opts.capitalize {
                let mut chars = variation.as_str().chars();
                let first = chars.next().unwrap().to_uppercase().to_string();
                password.push_str(&first);
                password.push_str(&chars.as_str());
            } else {
                password.push_str(&variation);
            };
            password.push_str(&suffix);
            attempts.push(password.clone());

            cnt = cnt + 1;
            if cnt == t as i32 {
                let handles: Vec<thread::JoinHandle<_>> = (0..attempts.len() as i32)
                    .map(|i| {
                        let pass = attempts[i as usize].clone();
                        let file_bytes = bytes_vector.clone();
                        thread::spawn(move || {
                            let success = try_crack(&file_bytes, &pass);
                            if success {
                                let elapsed = now.elapsed();
                                println!("---------------------------");
                                println!("Password found: {}", pass);
                                println!("Time elapsed: {:?}", elapsed);
                                println!("---------------------------");
                                std::process::exit(0);
                            }
                        })
                    })
                    .collect();
                for h in handles {
                    h.join().unwrap();
                }
                attempts.clear();
                cnt = 0;
            }
            total = total + 1;
            if total % 10000 == 0 {
                let elapsed = now.elapsed();
                println!("{} passwords tried in {:?} ", total, elapsed);
            }
        });
    }
    println!("Password not found");
}

fn try_crack(bytes: &Vec<u8>, password: &String) -> bool {
    debug!("{}", password);
    match decrypt_from_bytes(bytes.to_vec(), &password) {
        Ok(decrypted) => match calamine::open_workbook_auto_from_rs(Cursor::new(decrypted)) {
            Ok(_) => true,
            Err(_e) => false,
        },
        Err(e) => panic!("Open Error: {}", e),
    }
}

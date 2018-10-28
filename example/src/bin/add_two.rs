//! CLI math tool

extern crate arithmancy;
extern crate getopts;

use std::process;
use std::env;

// Show short CLI spec
fn usage(brief : &str, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
}

// CLI entry point
fn main() {
  let args : Vec<String> = env::args().collect();

  let program : &str = args[0].as_ref();

  let brief = format!("Usage: {} [options]", program);

  let mut opts : getopts::Options = getopts::Options::new();
  opts.optopt("n", "integer", "increment an integer by two (required)", "VAL");
  opts.optflag("h", "help", "print usage info");

  match opts.parse(&args[1..]) {
    Err(_) => {
      usage(&brief, &opts);
      process::abort();
    },
    Ok(optmatches) => {
      if optmatches.opt_present("h") {
        usage(&brief, &opts);
        process::exit(0);
      } else if optmatches.opt_present("n") {
        let n : i64 = optmatches
          .opt_str("n")
          .unwrap()
          .parse()
          .expect("Error parsing integer");

        println!("{}", arithmancy::add_two(n));
      } else {
        usage(&brief, &opts);
        process::abort();
      }
    }
  }
}

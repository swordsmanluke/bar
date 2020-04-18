extern crate argparse;

mod bar;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::io::stdout;
use std::process::exit;
use crate::bar::{ vertical_bar, horizontal_bar };

fn main() {
    let mut vertical = false;
    let mut title: String = String::new();
    let mut size = 5;
    let mut pct = -1;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Print a horizontal or vertical bar");
        ap.refer(&mut pct)
            .add_argument("percent", Store,
                          "Percent complete in int[0-100]").required();

        ap.refer(&mut vertical)
            .add_option(&["-v", "--vertical"], StoreTrue,
                        "Print vertically");

        ap.refer(&mut size)
            .add_option(&["-s", "--size"], Store,
                        "Size of bar in characters");

        ap.refer(&mut title)
            .add_option(&["-t", "--title"], Store,
                        "(Optional) title for the bar");

        ap.parse_args_or_exit();

    }

    if pct < 0 || pct > 100 {
        println!("Invalid percentage. Must be between 0..100");
        exit(1);
    }

    let opt_title: Option<&String> = if title.is_empty() { None } else { Some(&title) };

    if vertical {
        println!("{}", vertical_bar(opt_title, pct, size));
    } else {
        println!("{}", horizontal_bar(opt_title, pct, size));
    }
}

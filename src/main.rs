extern crate argparse;

mod bar;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::io::stdout;
use std::process::exit;
use crate::bar::{vertical_bar, horizontal_bar, BarColor};

fn main() {
    let mut vertical = false;
    let mut percentage = false;
    let mut asc_color = false;
    let mut desc_color = false;
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

        ap.refer(&mut percentage)
            .add_option(&["-p", "--percent"], StoreTrue,
                        "Show Percentage");

        ap.refer(&mut asc_color)
            .add_option(&["-c", "--color"], StoreTrue,
                        "In Ascending Colors: 0-25: Red, 25-75: Yellow, 75-100: Green");

        ap.refer(&mut desc_color)
            .add_option(&["-C", "--inverse-color"], StoreTrue,
                        "In Descending Colors: 0-25: Green, 25-75: Yellow, 75-100: Red");

        ap.parse_args_or_exit();
    }

    if pct < 0 || pct > 100 {
        println!("Invalid percentage. Must be between 0..100");
        exit(1);
    }

    let bar_color: BarColor = match (asc_color, desc_color) {
        (true, false) => BarColor::Ascending,
        (_, true) => BarColor::Descending,
        _ => BarColor::None
    };

    let opt_title: Option<&String> = if title.is_empty() { None } else { Some(&title) };

    if vertical {
        println!("{}", vertical_bar(opt_title, pct, size, percentage, bar_color));
    } else {
        println!("{}", horizontal_bar(opt_title, pct, size, percentage, bar_color));
    }
}

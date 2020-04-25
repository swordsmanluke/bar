use std::ops::Add;
use colored::*;

const BLOCK: &str = "█";
const V_HALF: &str =  "▄";
const V_BOTTOM: &str =  "_";
const V_EMPTY: &str = "-";

const H_HALF: &str =  "▌";
const H_QUARTER: &str =  "▎";
const H_EMPTY: &str = "|";

#[derive(Clone, Copy)]
pub enum BarColor {
    None,
    Ascending,
    Descending
}

pub fn vertical_bar(title: Option<&String>, pct: i32, size: i32, percentage: bool, bar_color: BarColor) -> String {
    let mut blocks: Vec<String> = Vec::new();

    let full_cells = full_cells(pct, size);
    let last_cell_fullness = pct_for_last_cell(pct, size);
    let remaining_cells = size - (full_cells + if last_cell_fullness > 0 { 1 } else { 0 });
    let offset = match title {
        Some(t) => t.len() / 2,
        None => 0
    };

    if percentage { blocks.push( format!("%{:02}", pct)); }

    for _ in 0..full_cells { blocks.push(color(center(BLOCK, offset), pct, bar_color)); }

    match last_cell_fullness {
        1..=25   => blocks.push(color(center(V_BOTTOM, offset), pct,bar_color)),
        26..=75  => blocks.push(color(center(V_HALF, offset), pct,bar_color)),
        76..=100 => blocks.push(color(center(BLOCK, offset), pct,bar_color)),
        _ => {}
    };

    for _ in 0..remaining_cells { blocks.push(center(V_EMPTY, offset)) }

    match title {
        Some(t) => blocks.push(format!("{}", t.clone())),
        None => {()}
    }

    blocks.reverse();
    blocks.join("\n").to_string()
}

pub fn horizontal_bar(title: Option<&String>, pct: i32, size: i32, percentage: bool, bar_color: BarColor) -> String {
    let mut out: String = String::new();

    let full_cells = full_cells(pct, size);
    let last_cell_fullness = pct_for_last_cell(pct, size);
    let remaining_cells = size - (full_cells + if last_cell_fullness > 0 { 1 } else { 0 });

    match title {
        Some(t) => out += format!("{} ", t.clone()).as_str(),
        None => {()}
    }

    if percentage { out += format!(" %{:02} ", pct).as_str(); }

    for _ in 0..full_cells { out += color(BLOCK.to_string(), pct, bar_color).as_str(); }

    match last_cell_fullness {
        1..=25   => out += color(H_QUARTER.to_string(), pct, bar_color).as_str(),
        26..=75  => out += color(H_HALF.to_string(), pct, bar_color).as_str(),
        76..=100 => out += color(BLOCK.to_string(), pct, bar_color).as_str(),
        _ => {}
    };

    for _ in 0..remaining_cells { out += H_EMPTY }

    out
}

fn color(s: String, pct: i32, bc: BarColor) -> String {
    match bc {
        BarColor::Ascending => asc_color(s, pct),
        BarColor::Descending => desc_color(s, pct),
        BarColor::None => s
    }
}

fn asc_color(s: String, pct: i32) -> String {
    match pct {
        0..=49 => s.bright_red().to_string(),
        50..=74 => s.bright_yellow().to_string(),
        75..=100 => s.green().to_string(),
        _ => s
    }.to_string()
}

fn desc_color(s: String, pct: i32) -> String {
    match pct {
        0..=49 => s.green().to_string(),
        50..=74 => s.bright_yellow().to_string(),
        75..=100 => s.bright_red().to_string(),
        _ => s
    }
}

fn center(s: &str, offset: usize) -> String {
    format!("{0:>1$}", s, offset+1)
}

fn full_cells(pct: i32, size: i32) -> i32 {
    let cell_size  = size as f32 / 100.0;

    (pct as f32 * cell_size) as i32
}

fn pct_for_last_cell(pct: i32, size: i32) -> i32 {
    let cell_size  = size as f32 / 100.0;
    (100.0 * ((pct as f32 * cell_size) - full_cells(pct, size) as f32)) as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_bar_prints_title() {
        assert_eq!(vertical_bar(Some(&String::from("title")), 0, 5), "title\n");
    }

    #[test]
    fn full_cells_calculates_correct() {
        assert_eq!(full_cells(50, 5), 2);
    }

    #[test]
    fn pct_for_last_cell_calculates_correctly() {
        assert_eq!(pct_for_last_cell(50, 5), 50);
    }

    #[test]
    fn pct_for_last_cell_calculates_correctly_when_full() {
        assert_eq!(pct_for_last_cell(100, 5), 0);
    }

    #[test]
    fn pct_for_last_cell_calculates_correctly_when_almost_full() {
        assert_eq!(pct_for_last_cell(99, 5), 95);
    }

    #[test]
    fn pct_for_last_cell_calculates_correctly_for_internal_boundary() {
        assert_eq!(pct_for_last_cell(20, 5), 0);
    }

    #[test]
    fn center_formats_things_to_their_offset() {
        assert_eq!(center("-", 1), " -");
    }
}
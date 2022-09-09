use std::iter;

use ansi_term::Colour::Yellow;

const DIVIDER_LEN: usize = 60;

#[cfg(not(tarpaulin_include))]
pub fn print_welcome_text(version: &str) {
    log::info!("{}", Yellow.paint(get_rep_str('=')));
    let text = format!("candentia v{}", version);
    log::info!("{}", Yellow.paint(text));
    log::info!("{}", Yellow.paint("A cli tool for managing CT-Scan data"));
    log::info!("{}", Yellow.paint(get_rep_str('-')));
}

// pub fn print_divider() {
//     let divider = get_rep_str('-');
//     log::info!("{}", Yellow.paint(divider));
// }

fn get_rep_str(sym: char) -> String {
    iter::repeat(sym).take(DIVIDER_LEN).collect()
}

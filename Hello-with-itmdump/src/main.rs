#![deny(unsafe_code)]
#![no_main]
#![no_std]

use::{entry,iprinln,iprinln!};

![entry]
fn main() {
    let itm = aux5::init();

    iprintln!(mut itm.stim[0],"Hello from STMF303 through Itmdump");

    loop{}
}

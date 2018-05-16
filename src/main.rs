#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate lfclib;

use clap::Arg;
use lfclib::LuxaforContext;
use lfclib::consts;

fn main() {
    let matches : clap::ArgMatches = app_from_crate!()
        .arg(Arg::with_name("COLOUR")
            .help("Colour to set Luxafor flag to")
            .required(true)
            .index(1))
        .arg(Arg::with_name("LED")
            .help("Led to set Luxafor flag to")
            .index(2))
        .get_matches();

    let _colour : [u8;3] = match matches
        .value_of("COLOUR")
        .map(|v| v.to_lowercase()) {

        Some(val) => {
            match val.as_str() {
                "r" => [255, 0, 0],
                "o" => [255, 64, 0],
                "y" => [255, 255, 0],
                "g" => [0, 255, 0],
                "b" => [0, 0, 255],
                "i" => [128, 0, 255],
                "v" => [255, 0, 255],
                _ => [0, 0, 0]
            }
        },
        _ => [0u8, 0u8, 0u8]
    };

    let _led : u8 = match matches
        .value_of("LED")
        .map(|v| v.to_lowercase()) {
        
        Some(val) => {
            match val.as_str() {
                "a" => consts::led::ALL,
                "ba" => consts::led::BACK_ALL,
                "bb" => consts::led::BACK_BOTTOM,
                "bm" => consts::led::BACK_MIDDLE,
                "bt" => consts::led::BACK_TOP,
                "fa" => consts::led::FRONT_ALL,
                "fb" => consts::led::FRONT_BOTTOM,
                "fm" => consts::led::FRONT_MIDDLE,
                "ft" => consts::led::FRONT_TOP,
                _ => consts::led::ALL,
            }
        },
        _ => consts::led::ALL
    };

    let ctx = LuxaforContext::new().unwrap();
    let device = ctx.open_device(lfclib::consts::FULL_FLAG).unwrap();
    device.solid(_colour[0], _colour[1], _colour[2], _led).unwrap();
    // device.fade(_colour[0], _colour[1], _colour[2], _led).unwrap();
    // device.pattern().unwrap();
    // device.strobe(_colour[0], _colour[1], _colour[2], _led).unwrap();
    // device.wave(_colour[0], _colour[1], _colour[2], _led).unwrap();
}

mod blocks;

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use argh::FromArgs;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, PropMode};
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt as _;

use blocks::{
    battery::{battery_perc_block, battery_status_block},
    cpu::cpu_perc_block,
    date::datetime_block,
    keymap::keymap_block,
    mem::ram_perc_block,
    mpris::mpris_block,
    Block,
};

type BlockArg = (Block, &'static str);

#[derive(FromArgs)]
/// slstatus
struct Cli {
    #[argh(switch, short = 's')]
    /// print status to stdout
    sflag: bool,
}

fn set_window_name(
    conn: &RustConnection,
    window: u32,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Set the WM_NAME property
    conn.change_property8(
        PropMode::REPLACE,
        window,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        name.as_bytes(),
    )?;

    // Set the _NET_WM_NAME property for modern window managers
    let net_wm_name = conn.intern_atom(false, b"_NET_WM_NAME")?.reply()?.atom;
    conn.change_property8(
        PropMode::REPLACE,
        window,
        net_wm_name,
        AtomEnum::STRING,
        name.as_bytes(),
    )?;

    conn.flush()?;
    Ok(())
}

fn get_root_window(conn: &RustConnection) -> u32 {
    let setup = conn.setup();
    let screen = &setup.roots[0]; // Get the first screen
    screen.root
}

fn format_blocks(blocks: &[BlockArg]) -> String {
    // let res = String::new();
    blocks
        .iter()
        .map(|(block, fmt)| fmt.replace("{}", &block.show()))
        .collect::<Vec<String>>()
        .join("")
}

// TODO: catch kill signal to clean root windows name

fn main() {
    let cli: Cli = argh::from_env();

    let (conn, _screen_num) = RustConnection::connect(None).unwrap();
    let root = get_root_window(&conn);

    let args = [
        (mpris_block(), "{} | "),
        (keymap_block(), "{} | "),
        (battery_perc_block("BAT0"), "BAT: {}"),
        (battery_status_block("BAT0"), " {}"),
        (cpu_perc_block(), " | CPU: {}%"),
        (ram_perc_block(), " | MEM: {}%"),
        (datetime_block(), " | {}"),
    ];

    loop {
        let start = Instant::now();
        let r = format_blocks(&args);

        if cli.sflag {
            println!("{}", r);
        } else {
            set_window_name(&conn, root, &r).unwrap();
        }

        let elapsed = start.elapsed();
        let remaining = Duration::from_millis(1000).saturating_sub(elapsed);

        // println!("{:?}", remaining);
        sleep(remaining);

        // done = true;
    }
}

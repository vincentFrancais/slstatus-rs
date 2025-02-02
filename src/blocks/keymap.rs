use x11rb::protocol::xkb::ConnectionExt as _;
use x11rb::protocol::xkb::{self, GetNamesReply, GetStateReply};
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::xcb_ffi::XCBConnection;

use super::Block;

fn keymap() -> anyhow::Result<String> {
    // Connect to the X11 server
    let (conn, _) = XCBConnection::connect(None)?;

    // Initialize the XKB extension
    let xkb_ver = conn.xkb_use_extension(1, 0)?.reply()?;
    if !xkb_ver.supported {
        return Err(anyhow::anyhow!("XKB extension not supported"));
    }

    // Get the current keyboard state
    let device_id = xkb::ID::USE_CORE_KBD.into();
    let state_reply: GetStateReply = conn.xkb_get_state(device_id)?.reply()?;
    let layout_index: u8 = state_reply.group.into();

    // Get the layout names
    let names_reply: GetNamesReply = conn
        .xkb_get_names(device_id, xkb::NameDetail::GROUP_NAMES)?
        .reply()?;

    // Extract the layout name from the reply
    let layout_name = names_reply
        .value_list
        .groups
        .and_then(|groups| groups.get(layout_index as usize).copied())
        .and_then(|atom| conn.get_atom_name(atom).ok())
        .and_then(|name| name.reply().ok())
        .map(|name| String::from_utf8(name.name).unwrap())
        .unwrap_or_else(|| "unknown".to_string());

    match layout_name.as_str() {
        "English (US)" => Ok("us".into()),
        "French" => Ok("fr".into()),
        "unknown" => Ok("?".into()),
        _ => Err(anyhow::anyhow!("wtf Oo")),
    }
}

pub fn keymap_block() -> Block {
    Block {
        func: Box::new(|| keymap().unwrap()),
    }
}

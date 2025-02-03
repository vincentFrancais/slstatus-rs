use std::time::Duration;

use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Proxy;
use dbus::{arg, blocking::Connection};

use super::{Block, BlockComponent};

const MPLAYER_NAMESPACE: &str = "org.mpris.MediaPlayer2";
const MPRIS_INTERFACE_NAME: &str = "org.mpris.MediaPlayer2.Player";
const MPRIS_PATH: &str = "/org/mpris/MediaPlayer2";

#[derive(Debug)]
#[allow(dead_code)]
struct PlayerProperties {
    title: String,
    artist: Vec<String>,
    album: String,
}

struct MprisPlayer {
    namespace: String,
}

impl MprisPlayer {
    fn get_proxy<'a>(&self, connection: &'a Connection) -> Proxy<'a, &'a Connection> {
        connection.with_proxy(
            self.namespace.clone(),
            MPRIS_PATH,
            Duration::from_millis(500),
        )
    }
    pub fn is_playing(&self, connection: &Connection) -> bool {
        let p = connection.with_proxy(&self.namespace, MPRIS_PATH, Duration::from_millis(500));
        let playback_status: String = p.get(MPRIS_INTERFACE_NAME, "PlaybackStatus").unwrap();

        playback_status == "Playing"
    }

    pub fn metadata(&self, connection: &Connection) -> PlayerProperties {
        let metadata: arg::PropMap = self
            .get_proxy(connection)
            .get(MPRIS_INTERFACE_NAME, "Metadata")
            .unwrap();

        let artist = if let Some(artist_variant) = metadata.get("xesam:artist") {
            // Safely cast the Variant into an Array of Strings
            artist_variant
                .0
                .as_iter()
                .map(|array| {
                    let mut vec = Vec::new();
                    for item in array {
                        if let Some(artist) = item.as_str() {
                            vec.push(artist.to_string());
                        }
                    }
                    vec
                })
                .unwrap_or_default()
        } else {
            vec![]
        };

        let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
        let album: Option<&String> = arg::prop_cast(&metadata, "xesam:album");

        PlayerProperties {
            title: title.unwrap().clone(),
            artist,
            album: album.unwrap().clone(),
        }
    }
}

// pub fn mpris_block() -> Block {
//     Block {
//         func: Box::new(|| {
//             let conn = Connection::new_session().unwrap();
//             let players: Vec<MprisPlayer> = get_players(&conn)
//                 .into_iter()
//                 .filter(|x| x.is_playing(&conn))
//                 .collect();

//             match players.first() {
//                 Some(player) => {
//                     let props = player.metadata(&conn);
//                     let artists = props.artist.join(", ");
//                     format!("{:} - {:}", artists, props.title)
//                 }
//                 None => String::new(),
//             }
//         }),
//     }
// }

struct MprisPlayerBlock {
    conn: Connection,
}
impl MprisPlayerBlock {
    fn new() -> Self {
        Self {
            conn: Connection::new_session().unwrap(),
        }
    }

    fn get(&self) -> String {
        let players: Vec<MprisPlayer> = get_players(&self.conn)
            .into_iter()
            .filter(|x| x.is_playing(&self.conn))
            .collect();

        match players.first() {
            Some(player) => {
                let props = player.metadata(&self.conn);
                let artists = props.artist.join(", ");
                format!("{:} - {:}", artists, props.title)
            }
            None => String::new(),
        }
    }
}

impl BlockComponent for MprisPlayerBlock {
    fn call(&mut self) -> String {
        self.get()
    }
}

pub fn mpris_block() -> Block {
    Block::new(MprisPlayerBlock::new())
}

fn get_players(conn: &Connection) -> Vec<MprisPlayer> {
    let proxy = &conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .unwrap();

    let players: Vec<MprisPlayer> = names
        .into_iter()
        .filter(|x| x.starts_with(MPLAYER_NAMESPACE))
        .map(|x| MprisPlayer { namespace: x })
        .collect();

    players
}

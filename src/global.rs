use crate::web;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use phf::phf_map;
use tokio::sync as tsync;
use log::info;


pub struct SharedData {
    pub sunset_time: Arc<Mutex<time::OffsetDateTime>>,
    pub root_path: String,
    pub evening_leaderboard: Arc<tsync::Mutex<HashMap<u64, u16>>>,
    pub first_ge_sent: Arc<AtomicBool>
}

impl SharedData {
    pub async fn new() -> Self {
        let mut exec_path = std::env::current_exe().expect("couldnt get current executable path");
        exec_path.pop();
        if cfg!(debug_assertions) {
            exec_path.pop(); exec_path.pop();
        }
        let assets_path = exec_path.to_string_lossy().to_string();

        let leaderboard: HashMap<u64, u16> = {
            let path_string = format!("{}/assets/leaderboard.bin", assets_path);
            let path = std::path::Path::new(&path_string);

            if !path.try_exists().expect("file checking error") {
                info!("new leaderboard file created");

                HashMap::new()
            } else {
                info!("leaderboard read from file");

                let bytes = std::fs::read(format!("{}/assets/leaderboard.bin", assets_path)).expect("couldnt read leaderboard file");
                rmp_serde::decode::from_slice(&bytes).expect("couldnt deserialize leaderboard")
            }
        };

        SharedData {
            sunset_time: Arc::new(Mutex::new(web::get_sunset_time().await.unwrap())),
            root_path: assets_path,
            evening_leaderboard: Arc::new(tsync::Mutex::new(leaderboard)),
            first_ge_sent: Arc::new(AtomicBool::new(false))
        }
    }
}

pub static EVENING_MOTD: &[&str] = &[
    "evening good?",
    "yawn.",
    "boo",
    "WHATEVER YOU DO, DO NOT LOOK AT THE MOON",
    "the sun will rise once again.",
    "DAY AND NIGHT, DAY AND NIGHT",
    "jkhgkdfhgskjldahfgkljsdfhgjklsdhfgjk",
    "mun pona",
    "toki a!",
    "Innate Technique: Sunset Blossom",
    "I didn’t know you could go this far… EVENING SUN!!!!",
    "You were magnificient, sunset. I shall never forget you for as long as I live.",
    "With this sunset, I summon...",
    "1% SUN 99% SET",
    "Are you the sun because you set everyday, or do you set everyday because you are the sun?",
    "Eight-Handled Sun Divergent Evening Divine General Eveningbot:",
    "the evening knows when it is by knowing when it isnt",
];

pub static NIGHT_MOTD: &[&str] = &[
    "the voices are coming.",
    "how many eepers does it change to take a log by bolb? none, their to busy ???? their body pillow",
    "who up shlombing rn",
    ":goofyskull:",
    "why are you up? disgusting.",
    "gay sex",
    "i hope mom doesnt find me playing on my ds",
    "estrogen injection hours",
    "you should totally knock on the walls your neighbours will appreciate it",
    "meow",
    "lets cant sleep together :3",
    "comrade, we must go hunt for business students",
    "yooyokokkokokoiyoykoyoyitoyitoykoyoykoykoyyoyoyoyokokokoykykyoyoyoyoykyoyyyyy",
    "THE LONELY STONER FREES HIS MIND AT NIGHT, MIND AT NIGHT",
    "THE MOON LOOKS BEAUTIFUL TONIGHT. YOU SHOULD GO LOOK.",
    "tenpo pimeja 🌃",
    "where are my programming socks",
    "Reverse Cursed Technique: Sunrise Wilting",
    "Domain Expansion: Restful Realm",
    "Throughout the beds and the pillows, I alone am the insomniac one",
    "You're eepy, sleep proud.",
    "Those who had inherited that curse, the one who could not fully leave it behind. They would all now bear witness, to the flesh of the one who cannot sleep, and their overwhelming brain fog.",
];

pub static GOOD_EVENINGS: &[&str] = &[
    "good evening",
    "dobry wieczor",
    "dobry wieczór",
    "tenpo pimeja pona",
    "pimeja pona",
    "buenas noches",
    "bonsoir",
    "こんばんは",
    "こんばん",
    "konbanwa",
    "konbanha",
    "konban",
    "晚上好",
    "晚安",
    "wan3 shang4 hao3",
    "wan shang hao",
    "wan4 an1",
    "wan an",
    "guten abend",
    "tráthnóna maith",
    "tráthnona maith",
    "trathnóna maith",
    "trathnona maith",
    "goejûn",
    "goejun",
    "gott kveld",
];

pub static EASTER_EGG_REACTS: phf::Map<&str, &str> = phf_map!{
    "good morning" => "<a:nerdo:1218307823549546497>",
    "kijetesantakalu" => "<:kijetesantakalu:1218305634563264572>",
    "lesbiab" => "<:pls:1218307863613673573>",
    "ana" => "<:ourdictator:1246936494548062302>",
    "niko" => "<:ourdictator:1246936494548062302>",
    "our dictator" => "<:ourdictator:1246936494548062302>",
    "benevolent dictator for life" => "<:ourdictator:1246936494548062302>",
    "shroom" => "<a:mushroomdance:1218307936271728680>",
    "soko" => "<a:mushroomdance:1218307936271728680>",
    "grzyb" => "<a:mushroomdance:1218307936271728680>",
    "tiocfaidh ár lá" => "🇮🇪", // ie flag
    "tiocfaidh ar la" => "🇮🇪",
    "egg" => "<a:eggblush:1218305920119865484>",
    "meow" => "<a:catkiss:1218306966301184040>",
    "mrew" => "<a:catkiss:1218306966301184040>",
    "mrow" => "<a:catkiss:1218306966301184040>",
    "mraw" => "<a:catkiss:1218306966301184040>",
    "nya" => "<a:catkiss:1218306966301184040>",
    "nja" => "<a:catkiss:1218306966301184040>",
    "moo" => "<a:krowa:1218306885824807103>",
    "whar" => "<:whar:1246955200200048703>",
};

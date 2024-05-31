use crate::web;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
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

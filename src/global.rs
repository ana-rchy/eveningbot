use crate::web;
use log::info;
use phf::phf_map;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tokio::sync as tsync;

pub struct SharedData {
    pub sunset_time: Arc<Mutex<time::OffsetDateTime>>,
    pub root_path: String,
    pub evening_leaderboard: Arc<tsync::Mutex<HashMap<u64, u16>>>,
    pub first_ge_sent: Arc<AtomicBool>,
}

impl SharedData {
    pub async fn new() -> Self {
        let mut exec_path = std::env::current_exe().expect("couldnt get current executable path");
        exec_path.pop();
        if cfg!(debug_assertions) {
            exec_path.pop();
            exec_path.pop();
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

                let bytes = std::fs::read(format!("{}/assets/leaderboard.bin", assets_path))
                    .expect("couldnt read leaderboard file");
                rmp_serde::decode::from_slice(&bytes).expect("couldnt deserialize leaderboard")
            }
        };

        SharedData {
            sunset_time: Arc::new(Mutex::new(web::get_sunset_time().await.unwrap())),
            root_path: assets_path,
            evening_leaderboard: Arc::new(tsync::Mutex::new(leaderboard)),
            first_ge_sent: Arc::new(AtomicBool::new(false)),
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
    "With this sunset, I summon...\nEIGHT-HANDLED SUN DIVERGENT GAMEDEV DIVINE GENERAL EVENINGBOT",
    "1% SUN 99% SET",
    "Are you the sun because you set everyday, or do you set everyday because you are the sun?",
    "the evening knows when it is by knowing when it isnt",
    "Sponsored by Pharmakokinetiks. Get 15% off on your next death by using code EVENINGBOT in the link on my profile page.",
    "EVENING IS NOW IS YOU",
    "7:27 when the sun sets",
    "hi jamie :)",
    "pontal 2",
    "why dont other countries have tayto? i feel bad for them",
    "*smokes weed*",
    "do not the sun",
    "mrew",
    "hogging cs++ servers since 2024",
    "SHADERS ARE JUST GPU CODE",
    "game dev tips: stop prematurely optimizing",
    "game dev tips: dont make an mmo unless youre insane",
    "game dev tips: netcode is hard",
    "game dev tips: my wife left me",
    "game dev tips: add comments to every single line so you can be 100% sure you understand everything",
    "game dev tips: learn art",
    "game dev tips: switch to neovim",
    "game dev tips: get an autism diagnosis",
    "game dve tip5: gel an dylsexio diaqnosis",
    "game dev tips: stop writing bad code",
    "try out compact claustrophobia",
    "game dev tips: state machines are so fucking useful",
    "game dev tips: add `return 0;` to main, EVERY SINGLE TIME.",
    "game dev tips: \"security through obscurity\" doesnt work",
    "game dev tips: if youre gonna commit to making a game, make sure its a game/genre you *actually* wanna make",
    "game dev tips: make CIA method blackpowder",
    "perfect time for some xonotic",
    "portal 3",
    "Just went number 3",
    "unity? more like, uni pee ahah",
    "Stand ready for my arrival, sun.",
    "\\*kicks you in the balls\\*",
    "```\n     REMEMBER:\nBIG DOG IS WATCHING\n```",
    "lesbiab,,",
    "Sent from my iPhone",
    "I can't do this anymore.",
    "did i ever tell you the definition of insanity?",
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
    "eheheheh. eheheeh. eheheh. eheheheheheheh. eheh.",
    "I'm gonna do some push-ups",
    "by the twelve im a realm reborning",
    "\"To be a woman is to perform\" - Jerma",
    "gerg",
    "gronk.",
    "firebombing trinity since 2023",
    "you should play oneshot",
    "mod your 3ds",
    "DESTROY THE CRADLE OF LIFE.",
    "Eveningbot's quest: 500 hours of mind pumping action!",
    "*weeds smoke*",
    "remember kids: drugs are fun",
    "what if instead of eveningbot it was freakybot, and instead of sending messages i sent my source code 😳",
    "SPECIAL MISSION FROM COMMANDER GOON INCOMING",
    "I will\n*fuck*\n   you",
    "Favourite Spongebob Quote",
    "Fuck da Discord Ops",
    "email me          send me an email at\n\n-# thanks",
    "these Extra-Terrestrials are Jerk-a-Lerkin' on my Thing!",
    "straight up \"cubin it\"",
    "why do they call it go-dot if you come to it 😂",
];

pub static GOOD_EVENINGS: &[&str] = &[];

pub static EASTER_EGG_REACTS: phf::Map<&str, &str> = phf_map! {};

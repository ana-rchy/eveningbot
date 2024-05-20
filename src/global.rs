use std::sync::{Arc, Mutex};

pub struct SharedData {
    pub sunset_time: Arc<Mutex<time::OffsetDateTime>>
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
    "where are my programming socks"
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
    "wan an"
];

use std::collections::HashMap;
use std::io;
use std::fs;
use rmp_serde::encode;

fn main() {
    let mut leaderboard = HashMap::<u64, u16>::new();

    println!("enter entries in format [user_id],[count]");
    println!("enter nothing to exit");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("cant read input");
        if input == "\n" {
            std::process::exit(0);
        }

        let mut params = input.trim_end().split(",");
        let id = params.next().unwrap().parse::<u64>().unwrap();
        let count = params.next().unwrap().parse::<u16>().unwrap();
        leaderboard.insert(id, count);

        let leaderboard_bytes = encode::to_vec(&leaderboard).expect("couldnt serialize leaderboard");
        _ = fs::write("leaderboard.bin", leaderboard_bytes);
    }
}

use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use fes::{board::{GameState, ChessGame}, pgn::{read_pgn_file, StrIter}};

fn main() -> std::io::Result<()> {
    let game = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    match game {
        Some(game) => println!("game looks like:\n{}", game),
        None => println!("invalid")
    };
    let f = File::open("/mnt/bulking/chess/lichess_db_standard_rated_2022-12.pgn")?;
    let reader = BufReader::new(f);

    let mut game_count: u64 = 0;
    let mut key_count = HashMap::new();
    for game in read_pgn_file(&mut StrIter::new(&mut reader.lines().map(|line| match line {Ok(t) => t, Err(_) => "".to_owned()}))) {
        game_count += 1;
        for (k, _) in game.meta {
            if let Some(v) = key_count.get_mut(&k) {
                *v += 1;
            }
            else {
                key_count.insert(k, 1u64);
            }
        }
        if game_count % 100000 == 0 {
            println!("games {}", game_count)
        }
    }
    println!("games: {game_count}");
    println!("keys: {key_count:?}");
    Ok(())
}

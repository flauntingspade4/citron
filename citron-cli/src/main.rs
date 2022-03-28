// use chesty_core::{explore_line, hash, Board, Position};

use citron_core::{analysis::explore_line, move_gen::Move, Board, Position};
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("chesty-cli")
        .version("0.1")
        .author("Elliot W")
        .subcommand(
            SubCommand::with_name("analyse")
                .help("Gets the best move for a given FEN")
                .about("Gets the best move for a given FEN")
                .arg(
                    Arg::with_name("fen")
                        .takes_value(true)
                        .required(true)
                        .help("The input FEN"),
                )
                .arg(
                    Arg::with_name("depth")
                        .takes_value(true)
                        .help("The desired depth ply of the analysis (default of 8)"),
                )
                .arg(
                    Arg::with_name("explore")
                        .short("e")
                        .help("Whether the whole line should be explored"),
                ),
        )
        .subcommand(
            SubCommand::with_name("play")
                .help("Plays the game from a given FEN")
                .about("Plays the game from a given FEN")
                .arg(
                    Arg::with_name("fen")
                        .takes_value(true)
                        .long("fen")
                        .help("The input FEN"),
                )
                .arg(
                    Arg::with_name("depth")
                        .short("d")
                        .long("depth")
                        .takes_value(true)
                        .help("The desired depth ply of the analysis (default of 8)"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("analyse", Some(t)) => {
            let depth = if let Some(depth) = t.value_of("depth") {
                depth.parse().unwrap_or(8)
            } else {
                8
            };

            let fen = t.value_of("fen").unwrap();
            let board = Board::from_fen(fen).unwrap();

            let table = board.iterative_deepening_ply(depth);

            if t.is_present("explore") {
                explore_line(board, &table);
            } else {
                let best = table.get(&board.hash()).unwrap();

                // let (from, to) = best.best_move;
                println!(
                    "Best move in position: {:?} {}",
                    best.best_move,
                    best.evaluation.into_inner() as f64 / 100.
                );
            }
        }
        ("play", Some(t)) => {
            let depth = if let Some(depth) = t.value_of("depth") {
                depth.parse().unwrap_or(8)
            } else {
                8
            };

            let fen = t
                .value_of("fen")
                .unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0");
            let mut board = Board::from_fen(fen).unwrap();

            println!("{} {}", board.material, board.absolute_material);

            loop {
                let eval = board.iterative_deepening_ply(depth);
                let best = eval.get(&board.hash()).unwrap();
                println!(
                    "{:?} {}",
                    best.best_move,
                    best.evaluation.into_inner() as f64 / 100.
                );

                board = board.make_move(&best.best_move).unwrap();

                println!("{}", board);

                let (from, to) = get_positions();

                let played_move = Move::new(
                    from,
                    to,
                    board.kind_at(board.to_play(), from),
                    board.kind_at(!board.to_play(), to),
                );

                board = board.make_move(&played_move).unwrap();

                println!("{:?}", board);
            }
        }
        _ => panic!(),
    }
}

fn get_positions() -> (Position, Position) {
    let mut buf = String::new();

    loop {
        buf.clear();

        while let Err(e) = std::io::stdin().read_line(&mut buf) {
            eprintln!("{}", e);
        }

        let mut chars = buf.split(' ');

        if let Some(from) = chars.next() {
            if let Some(from) = Position::from_uci(from) {
                if let Some(to) = chars.next() {
                    if let Some(to) = Position::from_uci(to) {
                        return (from, to);
                    }
                }
            }
        }
    }
}

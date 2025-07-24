use tetrizz::beam_search::*;
use tetrizz::data::*;
use tetrizz::eval::Eval;
use tetrizz::genetic::*;

use rand::seq::SliceRandom;

use tetrizz::movegen::*;

fn gen_queue(bags: u32) -> (Piece, Vec<Piece>) {
    let mut rng = rand::rng();
    let bag = [
        Piece::I,
        Piece::J,
        Piece::L,
        Piece::O,
        Piece::S,
        Piece::T,
        Piece::Z,
    ];
    let mut queue: Vec<Piece> = vec![];
    for _ in 0..bags {
        let mut new_bag = bag.to_vec();
        new_bag.shuffle(&mut rng);
        queue.extend(new_bag);
    }
    (queue.remove(0), queue)
}

fn append_queue(queue: &mut Vec<Piece>, bags: u32) {
    let mut rng = rand::rng();
    let bag = [
        Piece::I,
        Piece::J,
        Piece::L,
        Piece::O,
        Piece::S,
        Piece::T,
        Piece::Z,
    ];
    for _ in 0..bags {
        let mut new_bag = bag.to_vec();
        new_bag.shuffle(&mut rng);
        queue.extend(new_bag);
    }
}

// fn perft(game: &Game, queue: &[Piece; 7], idx: usize, depth: usize) -> usize {
//     if depth == 1 {
//         return movegen_piece(&game.board, queue[idx]).len();
//     }

//     let mut nodes = 0;
//     for mv in movegen_piece(&game.board, queue[idx]) {
//         let mut next_game = game.clone();
//         next_game.advance(queue[idx], mv);
//         nodes += perft(&next_game, queue, idx + 1, depth - 1);
//     }

//     nodes
// }

// fn main() {
//     let game = Game::new(None);
//     let queue = [Piece::I, Piece::O, Piece::L, Piece::J, Piece::S, Piece::Z, Piece::T];

//     for d in 3..=6 {
//         let now = std::time::Instant::now();
//         let nodes = perft(&game, &queue, 0, d);
//         let elapsed = now.elapsed().as_micros() as usize;
//         println!("Depth: {d}  |  Nodes: {nodes}  |  Time: {}ms  |  NPS: {}", elapsed as f32 / 1000.0, nodes as f32 / (elapsed as f32 / 1000000.0));
//     }
// }

// fn main() {
//     let mut game = Game::new(None);
//     game.board.cols = [2047,1023,511,0,0,340,8062,16383,5119,2047];
//     movegen_piece(&game.board, Piece::Z);
// }

// fn main() {
//     run_genetic_algo();
// }

fn main() {
    let (test_hold, mut test_queue) = gen_queue(50);
    //let eval = Eval::from([124.09421, -367.82962, 306.19385, -213.37228, 25.347483, -389.50592, -255.44745, 357.6906, -31.861994, 318.49466, 7.4310007, 197.0811, -248.70837, 401.57187]);
    let eval = Eval::from([
        -79.400375,
        -55.564907,
        -125.680145,
        -170.41902,
        10.167948,
        -172.78625,
        -478.7291,
        86.84883,
        368.89203,
        272.57874,
        28.938646,
        -104.59018,
        -496.8832,
        458.29822,
    ]);
    let mut game = Game::new(Some(test_hold));

    let mut all_locations: Vec<PieceLocation> = vec![];

    for p in 0..20000 {
        if test_queue.len() < 50 {
            append_queue(&mut test_queue, 10);
        }
        let loc = search(&game, test_queue.clone(), &eval, 10, 3000);
        all_locations.push(loc);

        let mut outstr: Vec<String> = vec![];
        for y in (0..20).rev() {
            let mut vstr = String::new();
            for x in 0..10 {
                vstr.push_str(if (game.board.cols[x as usize] & (1 << y)) > 0 {
                    "🟩"
                } else if loc.blocks().iter().any(|(bx, by)| *bx == x && *by == y) {
                    if loc.spun {
                        "🟨"
                    } else {
                        "🟥"
                    }
                } else {
                    "⬜️"
                });
            }
            outstr.push(vstr);
        }

        let mut queue5 = test_queue.clone();
        queue5.truncate(5);

        outstr[5] += &format!("          b2b:            ⭐️ \x1b[1m{}\x1b[0m ⭐️", game.b2b);
        outstr[6] += &format!("          pieces placed:    {:?}", p + 1);
        outstr[7] += &format!("          board:            {:?}", game.board.cols);
        outstr[8] += &format!("          queue (next 5):   {:?}", queue5);
        outstr[9] += &format!("          hold piece:       {:?}", game.hold);

        outstr[11] += &format!("          placed piece:     {:?}", loc.piece);

        println!("\n\n\n\n\n\n\n\n{}", outstr.join("\n"));

        game.advance(test_queue[0], loc);
        if loc.piece == game.hold {
            game.hold = test_queue[0]
        }

        test_queue.remove(0);
        if game
            .board
            .cols
            .iter()
            .map(|col| 64 - col.leading_zeros())
            .max()
            .unwrap()
            > 18
        {
            break;
        }
        // std::thread::sleep(std::time::Duration::from_millis(200));
    }
    // println!("{}", serde_json::to_string(&all_locations).unwrap());
}

// [0.15875702, 0.016595798, 0.024426488, 0.25492343, -0.2874213, -0.008358666, -0.54286313, -0.5180494, 0.5129537, -0.007761954]

// [-79.400375, -55.564907, -125.680145, -170.41902, 10.167948, -172.78625, -478.7291, 86.84883, 368.89203, 272.57874, 28.938646, -104.59018, -496.8832, 458.29822]

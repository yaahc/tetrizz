use tetrizz::beam_search::*;
use tetrizz::data::*;
use tetrizz::eval::Eval;

use rand::seq::SliceRandom;

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

fn main() {
    let (test_hold, mut test_queue) = gen_queue(50);
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
                vstr.push_str(if (game.board.cols[x as usize].0 & 1 << y) > 0 {
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
        outstr[7] += &format!(
            "          p/b2b:            {:?}",
            ((p + 1) as f64) / (game.b2b as f64)
        );
        outstr[8] += &format!("          board:            {:?}", game.board.cols);
        outstr[9] += &format!("          queue (next 5):   {queue5:?}");
        outstr[10] += &format!("          hold piece:       {:?}", game.hold);

        outstr[12] += &format!("          placed piece:     {:?}", loc.piece);

        println!("\n\n\n\n\n\n\n\n{}", outstr.join("\n"));

        game.advance(test_queue[0], loc);
        if loc.piece == game.hold {
            game.hold = test_queue[0]
        }

        test_queue.remove(0);
        if game
            .board
            .cols
            .into_iter()
            .map(Column::height)
            .max()
            .unwrap()
            > 18
        {
            break;
        }
    }
}

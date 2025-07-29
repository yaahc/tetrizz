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
    // let eval = Eval::new(
    //     -79.400375,
    //     -55.564907,
    //     -125.680145,
    //     -170.41902,
    //     10.167948,
    //     -172.78625,
    //     -478.7291,
    //     86.84883,
    //     368.89203,
    //     272.57874,
    //     28.938646,
    //     -104.59018,
    //     -496.8832,
    //     458.29822,
    //     0.0,
    // );

    // let eval = Eval::new(
    //     -68.89784, -322.48935, 141.74829, -312.21017, -169.99406, -300.44138, -328.3075,
    //     -324.86316, -332.62, -262.97006, -195.73398, -96.3394, 213.57771, 294.96298, 284.95618,
    // );
    // let eval = Eval {
    //     max_height: 18.557714,
    //     max_height_half: -29.742006,
    //     max_height_quarter: -373.33258,
    //     total_holes: -441.13776,
    //     coveredness: -289.1855,
    //     row_transitions: -348.7071,
    //     depth4: 118.74084,
    //     dependencies: 219.70303,
    //     i_dependencies: -301.80637,
    //     spikes: -35.50817,
    //     concavity: -7.3576345,
    //     b2b_clear: -130.73671,
    //     b2b_deficit: -302.88788,
    //     b2b_diff: 356.4649,
    //     garbage_sent: 262.47885,
    //     garbage_efficiency: 100.0,
    //     combo: 100.0,
    //     well_column: [10.0, -50.0, 50.0, 40.0, 35.0, 35.0, 40.0, 50.0, -50.0, 10.0],
    // };
    //
    //
    // let weights = [
    //     -256.25583, -94.7653, 184.73387, 73.49247, -338.8251, -342.31683, -237.25266, 1.59794,
    //     -267.21558, -354.50406, 14.272699, 245.25119, -297.26068, 236.61342, -285.56296, 260.5679,
    //     215.24202,
    // ];
    // let col_weights = [312.08804, -778.25446, -376.71887, -336.72726, -204.00638];

    // let weights = [
    //     -32.773167, -35.73379, 232.07043, -373.92145, -55.489117, -160.79543, -353.53836,
    //     -192.42172, 288.0571, 349.1388, 24.693245, -142.15094, -48.932167, 286.84805, 172.23831,
    //     -352.90686, 385.29013,
    // ];
    // let col_weights = [-331.43127, -819.5078, 46.861195, 381.66214, -265.89145];
    // let weights = [
    //     -142.78696, -229.56374, 134.66144, 166.14958, -140.25345, -571.7986, -121.33763,
    //     -129.97269, 5.590478, -236.41142, 21.473736, -243.67111, -165.0492, 322.16586, 193.08154,
    //     -362.48047, -319.7668,
    // ];
    // let col_weights = [-284.23538, 408.4806, 83.88457, 845.9585, 172.254];

    // let weights = [
    //     -114.78176, 285.88263, 21.544422, 223.10156, -319.68512, -429.577, -299.8005, -210.70595,
    //     45.99, -97.821785, -28.65085, 63.409985, -403.26163, 224.14938, -283.94058, -213.94945,
    //     -279.3925,
    // ];
    // let col_weights = [119.340836, -260.7089, -694.892, 6.743904, -659.4454];

    // let weights = [
    //     -156.30476, -50.985424, -267.20517, 156.0283, -449.7257, -389.22397, 86.1335, -400.99854,
    //     217.25882, -121.46938, 395.35342, 77.565094, -48.246536, 304.46005, -51.386143, 62.19102,
    //     171.60942,
    // ];
    // let col_weights = [85.0956, -313.80145, -530.0633, -577.9823, -528.44745];
    //0
    // let col_weights = [289.39072, -522.69211, 564.7843, 389.1356, 280.37125];
    let col_weights = [-81.37885, -513.7885, 406.89426, 325.5154, 244.13655];

    let eval = Eval::new(
        -387.9232,
        304.60135,
        -148.4156,
        -60.793552,
        -318.75525,
        -294.6283,
        155.52512,
        -19.705757,
        -341.16006,
        -355.12494,
        106.10226,
        -1066.95909,
        310.3096,
        114.541352,
        255.449722,
        -369.87628,
        col_weights,
    );

    let mut game = Game::new(Some(test_hold));

    let mut all_locations: Vec<PieceLocation> = vec![];
    let mut garbage_sent = 0;

    for p in 0..50 {
        if test_queue.len() < 50 {
            append_queue(&mut test_queue, 10);
        }
        let loc = search(&game, test_queue.clone(), &eval, 15, 3000);

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

        let info = game.advance(test_queue[0], loc);

        garbage_sent += info.garbage_sent;
        outstr[12] += &format!(
            "          garbage sent:     {} ({})",
            garbage_sent, info.garbage_sent
        );

        println!("\n\n\n\n\n\n\n\n{}", outstr.join("\n"));
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

pub mod beam_search;
pub mod data;
pub mod eval;
// pub mod genetic;
pub mod movegen;

#[cfg(test)]
mod tests {
    use super::data::*;
    use super::movegen::*;

    #[test]
    fn movegen() {
        let mut game = Game::new(None);
        game.board.cols = [
            Column(7),
            Column(127),
            Column(31),
            Column(31),
            Column(31),
            Column(1),
            Column(0),
            Column(15),
            Column(15),
            Column(15),
        ];
        println!(
            "{:?}",
            movegen_piece(&game.board, Piece::T)
                .iter()
                .map(|x| format!(
                    "({},{},{}),",
                    x.x,
                    x.y,
                    x.rotation as i8 + (if x.spun { 100 } else { 0 })
                ))
                .collect::<String>()
        );
    }
}

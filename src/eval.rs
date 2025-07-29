use crate::data::*;
use rand::Rng;

pub struct Eval {
    pub max_height: f32,
    pub max_height_half: f32,
    pub max_height_quarter: f32,
    pub total_holes: f32,
    pub coveredness: f32,
    pub row_transitions: f32,
    pub depth4: f32,
    pub dependencies: f32,
    pub i_dependencies: f32,
    pub spikes: f32,
    pub concavity: f32,
    pub waste_t: f32,
    pub tst: f32,
    pub tsd: f32,
    pub garbage_efficiency: f32,
    pub combo: f32,
    pub well_column: [f32; 10],
}

impl Eval {
    pub const PARAMS: usize = 16;
    pub fn new(
        max_height: f32,
        max_height_half: f32,
        max_height_quarter: f32,
        total_holes: f32,
        coveredness: f32,
        row_transitions: f32,
        depth4: f32,
        dependencies: f32,
        i_dependencies: f32,
        spikes: f32,
        concavity: f32,
        waste_t: f32,
        tst: f32,
        tsd: f32,
        garbage_efficiency: f32,
        combo: f32,
        half_well_column: [f32; 5],
    ) -> Self {
        let mut well_column = [0.0; 10];
        let mut rev_half_well_column = half_well_column.clone();
        let (left, right) = well_column.split_at_mut(half_well_column.len());
        rev_half_well_column.reverse();
        left.copy_from_slice(&half_well_column);
        right.copy_from_slice(&rev_half_well_column);

        Self {
            max_height,
            max_height_half,
            max_height_quarter,
            total_holes,
            coveredness,
            row_transitions,
            depth4,
            dependencies,
            i_dependencies,
            spikes,
            concavity,
            waste_t,
            tsd,
            tst,
            garbage_efficiency,
            combo,
            well_column,
        }
    }

    pub fn from(arr: [f32; Self::PARAMS], half_well_column: [f32; 5]) -> Self {
        Self::new(
            arr[0],
            arr[1],
            arr[2],
            arr[3],
            arr[4],
            arr[5],
            arr[6],
            arr[7],
            arr[8],
            arr[9],
            arr[10],
            arr[11],
            arr[12],
            arr[13],
            arr[14],
            arr[15],
            half_well_column,
        )
    }

    // new eval, largely copied from cc2 :3 (sorry mk im still learning)
    pub fn eval(&self, game: &Game, piece: Piece, info: &PlacementInfo) -> f32 {
        // height
        let heights: [i32; 10] = game.board.cols.map(|c| c.height() as _);

        let max_height = game.board.max_height_col();
        let max_height_half = max_height.max(10) - 10;
        let max_height_quarter = max_height.max(15) - 15;

        let total_holes = game
            .board
            .cols
            .iter()
            .map(|&c| {
                let h = c.height() as u32;
                let under = (1 << h) - 1;
                (!c.0 & under).count_ones()
            })
            .sum::<u32>();

        let mut coveredness = 0;
        for &c in &game.board.cols {
            let h = 64 - c.0.leading_zeros();
            let under = (1 << h) - 1;
            let mut holes = !c.0 & under;
            while holes != 0 {
                let y = holes.trailing_zeros();
                coveredness += h - y;
                holes &= !(1 << y);
            }
        }

        let row_transitions = game
            .board
            .cols
            .windows(2)
            .map(|c| (c[0].0 ^ c[1].0).count_ones())
            .sum::<u32>();

        let (well_col, well_height) = game
            .board
            .cols
            .iter()
            .enumerate()
            .min_by_key(|&(_, h)| h)
            .unwrap();
        let almost_full_lines = game.board.cols[0..well_col].iter().fold(!0, |a, b| a & b.0);
        let almost_full_lines = game.board.cols[well_col + 1..]
            .iter()
            .fold(almost_full_lines, |a, b| a & b.0);
        let depth4 = (almost_full_lines >> well_height.0).trailing_ones();

        // dependencies, spikes

        let mut dependencies = 0;
        let mut i_dependencies = 0;
        let mut spikes = 0;
        let mut concavity = 0;

        for x in 0..10 {
            if x == well_col {
                continue;
            }

            let a = heights.get(x - 1).copied().unwrap_or(99);
            let b = heights[x];
            let c = heights.get(x + 1).copied().unwrap_or(99);

            dependencies += (a - 1 > b && c - 1 > b) as i32;
            i_dependencies += (a - 2 > b && c - 2 > b) as i32;
            spikes += (a + 1 < b && c + 1 < b) as i32;
            concavity += a - 2 * b + c;
        }

        let mut tsd = false;
        let mut tst = false;
        let mut waste_t = false;

        if piece == Piece::T {
            if info.spin {
                match info.lines_cleared {
                    2 => tsd = true,
                    3 => tst = true,
                    _ => {}
                }
            }
            if !(tsd || tst) {
                waste_t = true;
            }
        }

        // other stuff
        let garbage_efficiency = if info.lines_cleared > 0 {
            if !(info.spin && piece == Piece::T) {
                return -100000.0;
            }
            info.garbage_sent / info.lines_cleared
        } else {
            0
        };

        // final part: add some rng so can encourage exploration

        let res = self.max_height * max_height as f32
            + self.max_height_half * max_height_half as f32
            + self.max_height_quarter * max_height_quarter as f32
            + self.total_holes * total_holes as f32
            + self.coveredness * coveredness as f32
            + self.row_transitions * row_transitions as f32
            + self.depth4 * depth4 as f32
            + self.dependencies * dependencies as f32
            + self.i_dependencies * i_dependencies as f32
            + self.spikes * spikes as f32
            + self.concavity * concavity as f32
            + self.waste_t * waste_t as u8 as f32
            + self.tst * tst as u8 as f32
            + self.tsd * tsd as u8 as f32
            + self.garbage_efficiency * garbage_efficiency as f32
            + self.well_column[well_col] * depth4 as f32;
        //+ self.combo * game.combo as f32;

        let noise = {
            let mut rng = rand::rng();
            let noise = 0.03 * rng.random_range(-res.abs()..=res.abs());
            noise
        };

        // higher is better
        {
            res + noise
        }
    }
}

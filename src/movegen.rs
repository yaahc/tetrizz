use crate::data::*;
use std::fmt;

const FULL_HEIGHT: u64 = (1 << 40) - 1;

pub fn movegen(game: &Game, next: Piece) -> Vec<PieceLocation> {
    let mut next_pieces = movegen_piece(&game.board, next);
    let mut hold_pieces = movegen_piece(&game.board, game.hold);
    next_pieces.append(&mut hold_pieces);
    next_pieces
}

pub fn movegen_piece(board: &Board, piece: Piece) -> Vec<PieceLocation> {
    const ROT: [Rotation; 4] = [
        Rotation::Up,
        Rotation::Right,
        Rotation::Down,
        Rotation::Left,
    ];
    const PAIRS: [[usize; 3]; 4] = [[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]];

    let mut maps = ROT.map(|r| CollisionMap::new(board, piece, r));

    if piece != Piece::O {
        let mut completed = [false, false, false, false];

        while completed != [true, true, true, true] {
            for i2 in 0..4 {
                let last = maps[i2].explored;
                let all_valid = maps[i2].all_valid;

                if last == all_valid {
                    completed[i2] = true;
                    continue;
                }

                for i1 in PAIRS[i2] {
                    let kicks = kicks(piece, ROT[i1], ROT[i2]);
                    let mut p1f = maps[i1].explored;
                    for (kx, ky) in kicks {
                        let mut mask = all_valid;
                        for x in 0..10 {
                            let c = p1f.get((x - kx) as usize).copied().unwrap_or(0);
                            let c = match ky < 0 {
                                true => c >> -ky,
                                false => c << ky,
                            };
                            mask[x as usize] &= c;
                            maps[i2].explored[x as usize] |= mask[x as usize];
                            maps[i2].spin_loc[x as usize] |= mask[x as usize];
                        }
                        for x in 0..10 {
                            let c = mask.get((x + kx) as usize).copied().unwrap_or(0);
                            let c = match ky < 0 {
                                true => c << -ky,
                                false => c >> ky,
                            };
                            p1f[x as usize] &= !c;
                        }
                    }
                }
                maps[i2].floodfill();
                if maps[i2].explored == last {
                    completed[i2] = true;
                }
            }
        }
    }

    for map in &mut maps {
        for x in 0..10 {
            map.explored[x] &= map.obstructed[x] << 1 | 1;
            map.spin_loc[x] &= map.obstructed[x] << 1 | 1;
        }
    }

    let mut new_maps: Vec<CollisionMap> = maps.into_iter().collect();

    if piece == Piece::S || piece == Piece::Z {
        for x in 0..10 {
            // down to up
            new_maps[Rotation::Up as usize].explored[x] |=
                new_maps[Rotation::Down as usize].explored[x] >> 1;
            new_maps[Rotation::Up as usize].spin_loc[x] |=
                new_maps[Rotation::Down as usize].spin_loc[x] >> 1;
            // left to right
            new_maps[Rotation::Right as usize].explored[x] |= new_maps[Rotation::Left as usize]
                .explored
                .get(x + 1)
                .copied()
                .unwrap_or(0);
            new_maps[Rotation::Right as usize].spin_loc[x] |= new_maps[Rotation::Left as usize]
                .spin_loc
                .get(x + 1)
                .copied()
                .unwrap_or(0);
        }
        new_maps.truncate(2);
    } else if piece == Piece::I {
        for x in 0..10 {
            // down to up
            new_maps[Rotation::Up as usize].explored[x] |= new_maps[Rotation::Down as usize]
                .explored
                .get(x + 1)
                .copied()
                .unwrap_or(0);
            new_maps[Rotation::Up as usize].spin_loc[x] |= new_maps[Rotation::Down as usize]
                .spin_loc
                .get(x + 1)
                .copied()
                .unwrap_or(0);
            // left to right
            new_maps[Rotation::Right as usize].explored[x] |=
                new_maps[Rotation::Left as usize].explored[x] << 1;
            new_maps[Rotation::Right as usize].spin_loc[x] |=
                new_maps[Rotation::Left as usize].spin_loc[x] << 1;
        }
        new_maps.truncate(2);
    } else if piece == Piece::O {
        new_maps.truncate(1);
    }

    let actual_spin: Vec<[u64; 10]> = match piece {
        Piece::T => {
            let mut s = [0u64; 10];
            for (x, item) in s.iter_mut().enumerate() {
                let left = board.cols.get(x - 1).map(|c| c.0).unwrap_or(FULL_HEIGHT);
                let right = board.cols.get(x + 1).map(|c| c.0).unwrap_or(FULL_HEIGHT);

                let c1 = left << 1 | 1;
                let c2 = right << 1 | 1;
                let c3 = right >> 1;
                let c4 = left >> 1;

                *item = (c1 & c2 & (c3 | c4)) | (c3 & c4 & (c1 | c2));
            }
            new_maps
                .iter()
                .map(|map| {
                    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| {
                        (s[x]
                            | (map.obstructed.get(x - 1).copied().unwrap_or(FULL_HEIGHT)
                                & map.obstructed.get(x + 1).copied().unwrap_or(FULL_HEIGHT)
                                & (map.obstructed[x] >> 1)))
                            & map.spin_loc[x]
                    })
                })
                .collect()
        }
        _ => new_maps
            .iter()
            .map(|map| {
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| {
                    map.obstructed.get(x - 1).copied().unwrap_or(FULL_HEIGHT)
                        & map.obstructed.get(x + 1).copied().unwrap_or(FULL_HEIGHT)
                        & (map.obstructed[x] >> 1)
                        & map.spin_loc[x]
                })
            })
            .collect(),
    };

    let mut positions: Vec<PieceLocation> = Vec::with_capacity(40);
    for (rot_i, map) in new_maps.iter().enumerate() {
        for x in 0..10 {
            let mut remaining = map.explored[x as usize];
            let mut spin_remaining = actual_spin[rot_i][x as usize];

            let mut plc = remaining
                & map
                    .obstructed
                    .get((x - 1) as usize)
                    .copied()
                    .unwrap_or(FULL_HEIGHT)
                & map
                    .obstructed
                    .get((x + 1) as usize)
                    .copied()
                    .unwrap_or(FULL_HEIGHT);

            let mut y = 0;
            while remaining != 0 {
                if remaining & 1 == 1 {
                    positions.push(PieceLocation {
                        piece,
                        rotation: ROT[rot_i],
                        spun: spin_remaining & 1 == 1,
                        possible_line_clear: plc & 1 == 1,
                        x,
                        y,
                    });
                }
                remaining >>= 1;
                spin_remaining >>= 1;
                plc >>= 1;
                y += 1;
            }
        }
    }

    positions
}

const fn kicks(piece: Piece, from: Rotation, to: Rotation) -> [(i8, i8); 6] {
    match piece {
        Piece::O => [(0, 0); 6], // just be careful not to rotate the O piece at all lol
        Piece::I => match (from, to) {
            (Rotation::Right, Rotation::Up) => {
                [(-1, 0), (-2, 0), (1, 0), (-2, -2), (1, 1), (-1, 0)]
            }
            (Rotation::Right, Rotation::Down) => {
                [(0, -1), (-1, -1), (2, -1), (-1, 1), (2, -2), (0, -1)]
            }
            (Rotation::Right, Rotation::Left) => {
                [(-1, -1), (0, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1)]
            }
            (Rotation::Down, Rotation::Up) => {
                [(-1, 1), (-1, 0), (-1, 1), (-1, 1), (-1, 1), (-1, 1)]
            }
            (Rotation::Down, Rotation::Right) => {
                [(0, 1), (-2, 1), (1, 1), (-2, 2), (1, -1), (0, 1)]
            }
            (Rotation::Down, Rotation::Left) => {
                [(-1, 0), (1, 0), (-2, 0), (1, 1), (-2, -2), (-1, 0)]
            }
            (Rotation::Left, Rotation::Up) => [(0, 1), (1, 1), (-2, 1), (1, -1), (-2, 2), (0, 1)],
            (Rotation::Left, Rotation::Right) => [(1, 1), (0, 1), (1, 1), (1, 1), (1, 1), (1, 1)],
            (Rotation::Left, Rotation::Down) => [(1, 0), (2, 0), (-1, 0), (2, 2), (-1, -1), (1, 0)],
            (Rotation::Up, Rotation::Right) => [(1, 0), (2, 0), (-1, 0), (-1, -1), (2, 2), (1, 0)],
            (Rotation::Up, Rotation::Left) => {
                [(0, -1), (-1, -1), (2, -1), (2, -2), (-1, 1), (0, -1)]
            }
            (Rotation::Up, Rotation::Down) => [(1, -1), (1, 0), (1, -1), (1, -1), (1, -1), (1, -1)],
            _ => panic!(), // this should never happen lol
        },
        _ => match (from, to) {
            (Rotation::Right, Rotation::Up) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2), (0, 0)],
            (Rotation::Right, Rotation::Down) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2), (0, 0)],
            (Rotation::Right, Rotation::Left) => [(0, 0), (1, 0), (1, 2), (1, 1), (0, 2), (0, 1)],
            (Rotation::Down, Rotation::Up) => [(0, 0), (0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0)],
            (Rotation::Down, Rotation::Right) => {
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2), (0, 0)]
            }
            (Rotation::Down, Rotation::Left) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2), (0, 0)],
            (Rotation::Left, Rotation::Up) => [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2), (0, 0)],
            (Rotation::Left, Rotation::Right) => {
                [(0, 0), (-1, 0), (-1, 2), (-1, 1), (0, 2), (0, 1)]
            }
            (Rotation::Left, Rotation::Down) => {
                [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2), (0, 0)]
            }
            (Rotation::Up, Rotation::Right) => {
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2), (0, 0)]
            }
            (Rotation::Up, Rotation::Left) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2), (0, 0)],
            (Rotation::Up, Rotation::Down) => [(0, 0), (0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)],
            _ => panic!(), // this should never happen lol
        },
    }
}

#[derive(Debug, Clone)]
pub struct CollisionMap {
    pub obstructed: [u64; 10],
    pub all_valid: [u64; 10],
    pub explored: [u64; 10],
    pub spin_loc: [u64; 10],
}

impl CollisionMap {
    pub fn new(board: &Board, piece: Piece, rotation: Rotation) -> Self {
        let mut obstructed = [0u64; 10];
        for (dx, dy) in rotation.rotate_blocks(piece.blocks()) {
            for x in 0..10 {
                let c = board
                    .cols
                    .get((x + dx) as usize)
                    .map(|c| c.0)
                    .unwrap_or(FULL_HEIGHT);
                let c = match dy < 0 {
                    true => !(!c << -dy),
                    false => c >> dy,
                };
                obstructed[x as usize] |= c;
            }
        }

        let max_height = board.cols.into_iter().map(Column::height).max().unwrap();
        let mut all_valid: [u64; 10] = [(1 << (max_height + 3)) - 1; 10];
        let mut explored = [1 << (max_height + 2); 10];
        for x in 0..10 {
            all_valid[x] &= !obstructed[x];
            explored[x] &= !obstructed[x];
        }

        let mut res = Self {
            obstructed,
            all_valid,
            explored,
            spin_loc: [0u64; 10],
        };
        res.floodfill();
        res
    }

    fn floodfill(&mut self) -> [u64; 10] {
        let mut last = [0u64; 10];
        let mut res = self.explored;
        while last != res {
            last = res;
            for x in 0..10 {
                let mut last_col = 0u64;
                while last_col != res[x] {
                    last_col = res[x];
                    res[x] |= (res[x] >> 1) & !self.obstructed[x];
                }
                res[x] |= (res.get(x - 1).copied().unwrap_or(0)
                    | res.get(x + 1).copied().unwrap_or(0))
                    & !self.obstructed[x];
            }
        }
        self.explored = res;
        res
    }
}

impl fmt::Display for CollisionMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let outstr = (0..20)
            .rev()
            .map(|x| {
                let explored = self
                    .explored
                    .iter()
                    .map(move |v| {
                        String::from(if (v & (1 << x)) == 0 {
                            "⬜️"
                        } else {
                            "🟩"
                        })
                    })
                    .collect::<Vec<String>>()
                    .join("");
                let all_valid = self
                    .all_valid
                    .iter()
                    .map(move |v| {
                        String::from(if (v & (1 << x)) == 0 {
                            "⬜️"
                        } else {
                            "🟩"
                        })
                    })
                    .collect::<Vec<String>>()
                    .join("");
                let obstructed = self
                    .obstructed
                    .iter()
                    .map(move |v| {
                        String::from(if (v & (1 << x)) == 0 {
                            "⬜️"
                        } else {
                            "🟩"
                        })
                    })
                    .collect::<Vec<String>>()
                    .join("");
                let spin_loc = self
                    .spin_loc
                    .iter()
                    .map(move |v| {
                        String::from(if (v & (1 << x)) == 0 {
                            "⬜️"
                        } else {
                            "🟩"
                        })
                    })
                    .collect::<Vec<String>>()
                    .join("");
                format!("{obstructed}     {all_valid}     {explored}     {spin_loc}")
            })
            .collect::<Vec<String>>();
        write!(f, "Obstructed               All valid                Explored                 Spin location\n{}", outstr.join("\n"))
    }
}

use std::collections::BinaryHeap;

use ordered_float::NotNan;
// use std::collections::BTreeSet;

use crate::data::*;
use crate::eval::Eval;
use crate::movegen::movegen;

#[derive(Clone, Debug)]
pub struct Node {
    pub score: NotNan<f32>,
    pub id: usize,
    pub game: Game,
    // pub moves: im::Vector<(PieceLocation, PlacementInfo)>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

// why are there two heaps? How does info not get left behind in one heap or the other?
// we are clearing the old one before we swap, then building placements ontop of it
pub fn search_results(
    root: &Game,
    search_loc: &[PieceLocation],
    queue: Vec<Piece>,
    eval: &Eval,
    depth: usize,
    width: usize,
) -> BinaryHeap<Node> {
    let mut heap: BinaryHeap<Node> = BinaryHeap::with_capacity(width + 1);
    // for each piece placement available for the next piece, insert into heap if the score is high
    // enough to justify insertion
    for (id, &loc) in search_loc.iter().enumerate() {
        let mut game = root.clone();
        let placement_info = game.advance(queue[0], loc);
        let score = eval.eval(&game, loc.piece, &placement_info);
        insert_if_better(
            &mut heap,
            Node {
                game,
                id,
                score: NotNan::new(score).unwrap(),
                // moves: im::vector![(loc, placement_info)],
            },
            width,
        );
    }
    let mut next_heap: BinaryHeap<Node> = BinaryHeap::with_capacity(width + 1);
    // then, for every other piece in the queue
    for next in queue.iter().take(depth).skip(1) {
        // for each placement based on the first piece
        // let mut seen: BTreeSet<Board> = BTreeSet::default();
        // let mut skipped = 0;
        for node in &heap {
            // find subsequent placements, and for each
            for loc in movegen(&node.game, *next) {
                let mut game = node.game.clone();
                let placement_info = game.advance(*next, loc);
                // let not_seen = seen.insert(game.board);
                // if !not_seen {
                //     skipped += 1;
                //     continue;
                // }
                if game.board.max_height_col() > 16 {
                    continue;
                }
                let score = eval.eval(&game, loc.piece, &placement_info);
                // let mut moves = node.moves.clone();
                // moves.push_back((loc, placement_info));
                insert_if_better(
                    &mut next_heap,
                    Node {
                        game,
                        id: node.id,
                        score: NotNan::new(score).unwrap() + node.score,
                        // moves,
                    },
                    width,
                );
            }
        }
        if next_heap.is_empty() {
            break;
        }
        // dbg!(skipped);
        heap.clear();
        std::mem::swap(&mut heap, &mut next_heap);
    }

    heap
}

// why are there two heaps? How does info not get left behind in one heap or the other?
// we are clearing the old one before we swap, then building placements ontop of it
pub fn search(
    root: &Game,
    queue: Vec<Piece>,
    eval: &Eval,
    depth: usize,
    width: usize,
) -> PieceLocation {
    // we get the locations for the next piece in the queue
    let search_loc = movegen(root, queue[0]);
    let heap = search_results(root, &search_loc, queue, eval, depth, width);

    search_loc[heap.into_iter().min().unwrap().id]
}

fn insert_if_better(heap: &mut BinaryHeap<Node>, node: Node, width: usize) {
    if heap.len() < width {
        heap.push(node);
    } else if let Some(worst) = heap.peek() {
        if node.score > worst.score {
            heap.pop();
            heap.push(node);
        }
    }
}

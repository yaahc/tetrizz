use std::collections::BinaryHeap;

use crate::data::*;
use crate::eval::Eval;
use crate::movegen::movegen;

#[derive(Clone, Debug)]
pub struct Node {
    pub game: Game,
    pub id: usize,
    pub score: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn search(
    root: &Game,
    queue: Vec<Piece>,
    eval: &Eval,
    depth: usize,
    width: usize,
) -> PieceLocation {
    let search_loc = movegen(root, queue[0]);
    let mut heap: BinaryHeap<Node> = BinaryHeap::with_capacity(width + 1);
    for (id, &loc) in search_loc.iter().enumerate() {
        let mut game = root.clone();
        let placement_info = game.advance(queue[0], loc);
        let score = eval.eval(root, &game, &placement_info);
        insert_if_better(&mut heap, Node { game, id, score }, width);
    }
    let mut next_heap: BinaryHeap<Node> = BinaryHeap::with_capacity(width + 1);
    for next in queue.iter().take(depth).skip(1) {
        for node in &heap {
            for loc in movegen(&node.game, *next) {
                let mut game = node.game.clone();
                let placement_info = game.advance(*next, loc);
                if game
                    .board
                    .cols
                    .iter()
                    .map(|x| 64 - x.leading_zeros() as i8)
                    .max()
                    .unwrap()
                    > 16
                {
                    continue;
                }
                let score = eval.eval(root, &game, &placement_info);
                insert_if_better(
                    &mut next_heap,
                    Node {
                        game,
                        id: node.id,
                        score,
                    },
                    width,
                );
            }
        }
        if next_heap.is_empty() {
            break;
        }
        heap.clear();
        std::mem::swap(&mut heap, &mut next_heap);
    }
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

mod board;

use std::sync::atomic::{AtomicUsize, Ordering};

use board::Board;

fn main() {
    let mut board = Board::default();

    println!("{}", evaluate(&mut board, 0));
}

static HIGHEST_DEPTH: AtomicUsize = AtomicUsize::new(0);

pub fn evaluate(board: &mut Board, depth: usize) -> bool {
    if depth > HIGHEST_DEPTH.load(Ordering::SeqCst) {
        HIGHEST_DEPTH.store(depth, Ordering::SeqCst);
        println!("New highest depth {}", depth);
    }
    if depth == 28 {
        return true;
    }

    let moves = board.all_legal_moves();

    for r#move in moves {
        board.play_move(r#move);

        if board.finished() {
            println!("{:?}", r#move);
            return true;
        }

        if evaluate(board, depth + 1) {
            println!("{:?}", r#move);
            return true;
        }

        board.play_move(r#move);
    }

    return false;
}

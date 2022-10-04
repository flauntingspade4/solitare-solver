mod board;

use board::Board;

fn main() {
    let mut board = Board::default();

    println!("{}", evaluate(&mut board, 0, &mut 0));
}

pub fn evaluate(board: &mut Board, depth: usize, highest_depth: &mut usize) -> bool {
    if depth > *highest_depth {
        *highest_depth = depth;
        println!("New highest depth {}", depth);
    }

    let moves = board.all_legal_moves();

    for r#move in moves {
        board.play_move(r#move);

        if board.finished() {
            println!("{:?}", r#move);
            return true;
        }

        if evaluate(board, depth + 1, highest_depth) {
            println!("{:?}", r#move);
            return true;
        }

        board.play_move(r#move);
    }

    return false;
}

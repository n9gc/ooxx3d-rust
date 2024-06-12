//! A module to output the game in terminal.
//!
//! 用来在命令行里输出游戏对局的模块。

use crate::{GameInfo, OperationInfo, Piece};

pub struct PieceStr {
    you: String,
    me: String,
    none: String,
}
pub struct Outer {
    piece_str: PieceStr,
}
impl Outer {
    pub fn new(piece_str: PieceStr) -> Outer {
        Outer { piece_str }
    }
    fn piece(&self, piece: &Piece) -> &String {
        match piece {
            Piece::Me => &self.piece_str.me,
            Piece::You => &self.piece_str.you,
            Piece::None => &self.piece_str.none,
        }
    }
    fn line(&self, game_info: &GameInfo) {
        std::print!("╬");
        for _j in 0..game_info.size {
            std::print!("═══╬");
        }
    }
    pub fn out_board(&self, game_info: &GameInfo, operation_info: &OperationInfo) {
        let size = game_info.size;
        let board = &game_info.board;
        let cursor = operation_info.cursor;
        for i in 0..size {
			self.line(game_info);
        	std::print!(" \n║ ");
            for j in 0..size {
                if (i, j) == cursor {
                    std::print!("{}", self.piece(&board[i][j]));
                } else {
                    std::print!("{}", self.piece(&board[i][j]));
                }
                std::print!("║ ");
            }
            std::print!("\n");
        }
		self.line(game_info);
    }
}

#[cfg(test)]
mod test {
    use crate::{GameInfo, OperationInfo, Piece};

    use super::{Outer, PieceStr};

	#[test]
	pub fn out_borad() {
		let game_info = GameInfo {
			board: vec![
				vec![Piece::You, Piece::Me, Piece::None],
				vec![Piece::Me, Piece::You, Piece::None],
				vec![Piece::You, Piece::None, Piece::Me],
			],
			size: 3,
		};
		let operation_info = OperationInfo {
			cursor: (2, 3),
		};
		let outer = Outer::new(PieceStr {
			you: String::from("OO"),
			me: String::from("XX"),
			none: String::from("  "),
		});
		outer.out_board(&game_info, &operation_info);
	}
}

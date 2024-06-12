//! A module to output the game in terminal.
//!
//! 用来在命令行里输出游戏对局的模块。

use crate::{GameInfo, OperationInfo, Piece};
use crossterm::style::Stylize;

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
    fn out(&self, info: &str, high: bool) {
        print!(
            "{}",
            if high {
                info.white().on_dark_red()
            } else {
                info.reset()
            }
        );
    }
    fn voidline(&self, game_info: &GameInfo) {
        for _j in 0..(2 * game_info.size + 10) {
            self.out(" ", false);
        }
		self.out("\n", false);
    }
    fn line(&self, game_info: &GameInfo) {
        self.out(" ╬", false);
        for _j in 0..game_info.size {
            self.out("═══╬", false);
        }
    }
    pub fn out_board(&self, game_info: &GameInfo, operation_info: &OperationInfo) {
        let size = game_info.size;
        let board = &game_info.board;
        let cursor = operation_info.cursor;
		self.voidline(game_info);
        for i in 0..size {
            self.line(game_info);
            self.out(" \n ║ ", false);
            for j in 0..size {
                let now_piece = self.piece(&board[i][j]);
                if (i, j) == cursor {
                    self.out(now_piece, true);
                } else {
                    self.out(now_piece, false);
                }
                self.out("║ ", false);
            }
            self.out("\n", false);
        }
        self.line(game_info);
        self.out(" \n", false);
		self.voidline(game_info);
		let str = "";
		print!("{}", str.reset());
    }
}

#[cfg(test)]
mod test {
    use crate::{GameInfo, OperationInfo, Piece};

    use super::{Outer, PieceStr};

    #[test]
    pub fn out_borad() {
        let game_info_0 = GameInfo {
            board: vec![
                vec![Piece::You, Piece::Me, Piece::None],
                vec![Piece::Me, Piece::You, Piece::None],
                vec![Piece::You, Piece::None, Piece::Me],
            ],
            size: 3,
        };
        let game_info_1 = GameInfo {
            board: vec![
                vec![Piece::You, Piece::Me, Piece::None, Piece::You],
                vec![Piece::You, Piece::Me, Piece::None, Piece::Me],
                vec![Piece::Me, Piece::You, Piece::None, Piece::None],
                vec![Piece::You, Piece::None, Piece::Me, Piece::Me],
            ],
            size: 4,
        };
        let outer = Outer::new(PieceStr {
            you: String::from("OO"),
            me: String::from("XX"),
            none: String::from("  "),
        });
        outer.out_board(&game_info_0, &OperationInfo { cursor: (1, 0) });
		print!("\n");
        outer.out_board(&game_info_1, &OperationInfo { cursor: (3, 2) });
    }
}

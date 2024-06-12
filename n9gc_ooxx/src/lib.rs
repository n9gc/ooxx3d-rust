pub enum Piece {
	You,
	Me,
	None,
}

pub struct GameInfo {
	pub size: usize,
	pub board: Vec<Vec<Piece>>,
}

pub struct OperationInfo {
	pub cursor: (usize, usize),
}
pub mod text_out;


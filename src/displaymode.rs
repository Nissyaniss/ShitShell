#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
	CarriageReturn,
	NewLineAndCarriageReturn,
	DisplayCommand,
	Backspace,
	Normal,
}

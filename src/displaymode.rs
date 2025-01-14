#[derive(Clone, Copy)]
pub enum Mode {
	CarriageReturn,
	NewLineAndCarriageReturn,
	DisplayCommand,
	Backspace,
	Normal,
}

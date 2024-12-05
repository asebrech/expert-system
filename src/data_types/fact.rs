#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Condition {
	START,
	AND,
	OR,
	XOR,
	END
}

pub struct Requirement<'a> {
	pub knowledge: &'a Knowledge<'a>,
	pub condition: Condition,
	pub should_exist: bool,
}

pub struct Knowledge<'a> {
	pub symbol: &'a str,
	pub fact: bool,
	pub requirements: Vec<Requirement<'a>>
}
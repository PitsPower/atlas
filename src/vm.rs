//! The ATLAS PC Virtual Machine. Used as a reference for the actual computer.

/// The number of registers.
const REGISTER_AMOUNT: usize = 16;

/// The different kinds of errors that can happen when parsing.
#[derive(Debug)]
enum ParsingErrorType {
	/// The operand given isn't valid.
	InvalidOperand(String),

	/// The line is blank.
	Blank,
	/// The instruction being executed isn't known.
	UnknownInstruction(String),
	/// The operands given to the instruction don't fit.
	InvalidOperands,
}

impl std::fmt::Display for ParsingErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParsingErrorType::InvalidOperand(op) => f.write_fmt(format_args!("Invalid operand \"{}\"", op)),
			ParsingErrorType::UnknownInstruction(instr) => f.write_fmt(format_args!("Invalid instruction \"{}\"", instr)),
			ParsingErrorType::InvalidOperands => f.write_fmt(format_args!("Invalid operands for instruction")),
			
			ParsingErrorType::Blank => unreachable!(),
		}
    }
}

/// A parsing error. Includes the type of error as well as things
/// such as the line number.
struct ParsingError {
	etype: ParsingErrorType,
	line_no: usize,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
			"Error on line {}: {}",
			self.line_no,
			self.etype,
		))
    }
}

/// The different kinds of instruction operands.
enum InstructionOperand {
	/// A register operand (e.g. $r1).
	Register(usize),
	/// An immediate value (e.g. 5).
	Immediate(u8),
}

impl std::str::FromStr for InstructionOperand {
    type Err = ParsingErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		if &s[0..2] == "$r" {
			let reg_idx = s[2..].parse()
				.ok().ok_or_else(|| ParsingErrorType::InvalidOperand(s.to_string()))?;

			if reg_idx < REGISTER_AMOUNT {
				Ok(InstructionOperand::Register(reg_idx))
			} else {
				Err(ParsingErrorType::InvalidOperand(s.to_string()))
			}
		} else {
			let res;

			if &s[0..2] == "0x" {
				res = u8::from_str_radix(s.trim_start_matches("0x"), 16);
			} else if &s[0..2] == "0b" {
				res = u8::from_str_radix(s.trim_start_matches("0b"), 2);
			} else {
				res = s.parse();
			}

			Ok(InstructionOperand::Immediate(
				res.ok().ok_or_else(|| ParsingErrorType::InvalidOperand(s.to_string()))?,
			))
		}
    }
}

/// The set of ATLAS assembly instructions.
#[derive(Debug)]
enum Instruction {
	/// Moves the data in the first register to the second register.
	MoveRegToReg(usize, usize),
	/// Move an immediate value into a register.
	MoveImmToReg(u8, usize),
}

impl std::str::FromStr for Instruction {
    type Err = ParsingErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut instr_parts = s.split(' ');

		let instr_name = instr_parts.next().unwrap().trim();

		match instr_name {
			"mov" => {
				let operand_str = instr_parts.collect::<String>();
				let mut operand_parts = operand_str.split(',');

				let op1: InstructionOperand = operand_parts.next().unwrap().trim().parse()?;
				let op2: InstructionOperand = operand_parts.next().unwrap().trim().parse()?;

				match (op1, op2) {
					(InstructionOperand::Register(r1), InstructionOperand::Register(r2)) =>
						Ok(Instruction::MoveRegToReg(r1, r2)),
						
					(InstructionOperand::Immediate(imm), InstructionOperand::Register(r2)) =>
						Ok(Instruction::MoveImmToReg(imm, r2)),

					_ => Err(ParsingErrorType::InvalidOperands),
				}
			},

			"" => Err(ParsingErrorType::Blank),
			_ => Err(ParsingErrorType::UnknownInstruction(instr_name.to_string())),
		}
    }
}

/// Parses an assembly program.
fn parse_assembly(assembly: String) -> Result<Vec<Instruction>, ParsingError> {
	let mut result = vec![];

	for (line_no, line) in assembly.trim().split('\n').enumerate() {
		match line.parse() {
			Err(ParsingErrorType::Blank) => {},
			Err(etype) => {
				return Err(ParsingError { etype, line_no: line_no + 1 });
			},
			Ok(res) => result.push(res),
		};
	}

	Ok(result)
}

/// The ATLAS PC virtual machine.
pub struct AtlasVM {
	pub registers: Vec<u8>,
}

impl AtlasVM {
	/// Returns a new [`AtlasVM`].
	pub fn new() -> Self {
		Self {
			registers: vec![0; REGISTER_AMOUNT],
		}
	}

	/// Runs an assembly program.
	pub fn run(&mut self, assembly: String) {
		let instrs = match parse_assembly(assembly) {
			Ok(instrs) => instrs,
			Err(err) => panic!("{}", err),
		};

		for instr in instrs {
			match instr {
				Instruction::MoveRegToReg(r1, r2) => {
					self.registers[r2] = self.registers[r1]
				},
				Instruction::MoveImmToReg(imm, r) => {
					self.registers[r] = imm
				},
			}
		}
	}
}

impl Default for AtlasVM {
    fn default() -> Self {
        Self::new()
    }
}

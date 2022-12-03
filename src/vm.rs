//! The ATLAS PC Virtual Machine. Used as a reference for the actual computer.

/// The number of registers.
const REGISTER_AMOUNT: usize = 16;
/// The size of RAM.
const RAM_SIZE: usize = 0x10000;	// Max for a 16-bit address

/// The different kinds of errors that can happen when parsing.
#[derive(Debug)]
enum ParsingErrorType {
	/// Invalid character.
	InvalidCharacter(char),
	/// Unexpected end of file.
	UnexpectedEOF,
	/// Unexpected token.
	UnexpectedToken(AssemblyTokenType),
	/// The instruction being executed isn't known.
	InvalidInstruction(String),
	/// The operands given to the instruction don't fit.
	InvalidOperands,
}

impl std::fmt::Display for ParsingErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidCharacter(char) => f.write_fmt(format_args!("Unexpected character '{}'", char)),
			Self::UnexpectedEOF => f.write_str("Unexpected end of file"),
			Self::UnexpectedToken(token) => f.write_fmt(format_args!("Unexpected token {}", token)),
			Self::InvalidInstruction(instr) => f.write_fmt(format_args!("Invalid instruction {}", instr)),
			Self::InvalidOperands => f.write_str("Invalid operands for instruction"),
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

/// The different kinds of tokens in assembly code.
#[derive(Clone, Debug, Eq, PartialEq)]
enum AssemblyTokenType {
	Colon,
	Comma,
	/// Left square bracket.
	LSquare,
	/// Right square bracket.
	RSquare,

	Identifier(String),
	Number(u32),
	Register(usize),
	String(String),
}

/// An assembly token. Includes the type of token as well as things
/// such as the line number.
struct AssemblyToken {
	ttype: AssemblyTokenType,
	line_no: usize,
}

impl std::fmt::Display for AssemblyTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Colon => f.write_str(":"),
			Self::Comma => f.write_str(","),
			Self::LSquare => f.write_str("["),
			Self::RSquare => f.write_str("]"),
			Self::Identifier(ident) => f.write_str(ident),
			Self::Number(num) => f.write_fmt(format_args!("{}", num)),
			Self::Register(reg) => f.write_fmt(format_args!("${}", reg)),
			Self::String(str) => f.write_fmt(format_args!("\"{}\"", str)),
		}
    }
}

/// The set of ATLAS assembly instructions.
#[derive(Debug)]
enum Instruction {
	/// Halts the program.
	Halt,
	/// Moves the data in the first register to the second register.
	MoveRegToReg(usize, usize),
	/// Move an immediate value into a register.
	MoveImmToReg(u16, usize),
	/// Moves the data in the first register to the address stored in the second register.
	MoveRegToRegAddr(usize, usize),
	/// Moves the data in the register to the address.
	MoveRegToImmAddr(usize, u16),
	/// Adds the first register and second register and stores the result in the third register.
	AddRegToReg(usize, usize, usize),
	/// Adds the immediate value to the first register and stores the result in the second register.
	AddImmToReg(usize, u16, usize),

	// Assembly directives. Not exactly instructions but whatever.

	/// Writes some data to the binary output.
	DataDirective(Vec<u8>),
}

impl Instruction {
	/// Returns the instruction as a machine code instruction.
	fn as_machine_code(&self) -> Vec<u8> {
		match self {
			Self::Halt => {
				vec![0x00]
			},
			Self::MoveRegToReg(r1, r2) => {
				vec![0x01, *r1 as u8, *r2 as u8]
			},
			Self::MoveImmToReg(imm, reg) => {
				let immb = imm.to_be_bytes();
				vec![0x02, *reg as u8, immb[0], immb[1]]
			},
			Self::MoveRegToRegAddr(r1, r2) => {
				vec![0x02, *r1 as u8, *r2 as u8]
			},
			Self::MoveRegToImmAddr(reg, imm) => {
				let immb = imm.to_be_bytes();
				vec![0x03, *reg as u8, immb[0], immb[1]]
			},
			Self::AddRegToReg(r1, r2, r3) => {
				vec![0x04, *r1 as u8, *r2 as u8, *r3 as u8]
			},
			Self::AddImmToReg(r1, imm, r2) => {
				let immb = imm.to_be_bytes();
				vec![0x05, *r1 as u8, immb[0], immb[1], *r2 as u8]
			},

			Self::DataDirective(data) => data.clone(),
		}
	}
}

/// Lexes an assembly program.
fn lex_assembly(assembly: String) -> Result<Vec<AssemblyToken>, ParsingError> {
	let mut result = vec![];
	let mut chars = assembly.chars().peekable();
	let mut line_no = 1;

	while let Some(&char) = chars.peek() {
		match char {
			'a'..='z' | 'A'..='Z' | '.' => {
				let mut identifier = String::new();

				while let Some(&char) = chars.peek() {
					if char.is_alphabetic() || char == '.' {
						identifier.push(chars.next().unwrap());
					} else {
						break;
					}
				}

				result.push(AssemblyToken {
					ttype: AssemblyTokenType::Identifier(identifier),
					line_no,
				});
			},

			'0'..='9' => {
				let mut base = 10;

				if char == '0' {
					chars.next().unwrap();

					if let Some(&char) = chars.peek() {
						match char {
							'b' => {
								base = 2;
								chars.next().unwrap();
							},
							'x' => {
								base = 16;
								chars.next().unwrap();
							},
							_ => {
								result.push(AssemblyToken {
									ttype: AssemblyTokenType::Number(0),
									line_no,
								});
								continue;
							},
						}
					}
				}

				let mut number = String::new();

				while let Some(&char) = chars.peek() {
					if char.is_digit(base) {
						number.push(chars.next().unwrap());
					} else {
						break;
					}
				}

				result.push(AssemblyToken {
					ttype: AssemblyTokenType::Number(u32::from_str_radix(number.as_str(), base).unwrap()),
					line_no,
				});
			},

			'$' => {
				chars.next().unwrap();

				let mut number = String::new();

				while let Some(&char) = chars.peek() {
					if char.is_ascii_digit() {
						number.push(chars.next().unwrap());
					} else {
						break;
					}
				}

				result.push(AssemblyToken {
					ttype: AssemblyTokenType::Register(number.parse().unwrap()),
					line_no,
				});
			},

			'"' => {
				chars.next().unwrap();

				let mut string = String::new();

				while let Some(&char) = chars.peek() {
					if char == '\\' {
						chars.next().unwrap();

						if let Some(char) = chars.next() {
							match char {
								'n' => string.push('\n'),
								'\\' => string.push('\\'),
								'"' => string.push('"'),
								_ => string.push(char),
							}
						} else {
							return Err(ParsingError { etype: ParsingErrorType::UnexpectedEOF, line_no });
						}
					} else if char != '"' {
						string.push(chars.next().unwrap());
					} else {
						break;
					}
				}

				if chars.next().is_none() {
					return Err(ParsingError { etype: ParsingErrorType::UnexpectedEOF, line_no });
				}

				result.push(AssemblyToken {
					ttype: AssemblyTokenType::String(string),
					line_no,
				});
			},

			_ => {
				match char {
					':' => result.push(AssemblyToken { ttype: AssemblyTokenType::Colon, line_no }),
					',' => result.push(AssemblyToken { ttype: AssemblyTokenType::Comma, line_no }),
					'[' => result.push(AssemblyToken { ttype: AssemblyTokenType::LSquare, line_no }),
					']' => result.push(AssemblyToken { ttype: AssemblyTokenType::RSquare, line_no }),
					'\n' => line_no += 1,
					' ' | '\t' => {},

					_ => {
						return Err(ParsingError { etype: ParsingErrorType::InvalidCharacter(char), line_no });
					},
				}

				chars.next().unwrap();
			},
		}
	}

	Ok(result)
}

/// Parses an assembly program.
fn parse_assembly(assembly: String) -> Result<Vec<Instruction>, ParsingError> {
	let mut result = vec![];
	
	let token_vec = lex_assembly(assembly)?;
	let mut tokens = token_vec.iter().peekable();

	while tokens.peek().is_some() {
		let token = tokens.next().unwrap();

		let ident = match &token.ttype {
			AssemblyTokenType::Identifier(ident) => ident.clone(),
			_ => return Err(ParsingError {
				etype: ParsingErrorType::UnexpectedToken(token.ttype.clone()),
				line_no: token.line_no,
			}),
		};

		let instr_name = ident.as_str();

		match instr_name {
			"mov" => {
				let op1 = tokens.next().ok_or(ParsingError {
					etype: ParsingErrorType::InvalidOperands,
					line_no: token.line_no,
				})?;

				let comma = tokens.next().ok_or(ParsingError {
					etype: ParsingErrorType::InvalidOperands,
					line_no: token.line_no,
				})?;

				if comma.ttype != AssemblyTokenType::Comma {
					return Err(ParsingError {
						etype: ParsingErrorType::UnexpectedToken(comma.ttype.clone()),
						line_no: token.line_no,
					});
				}

				let op2 = tokens.next().ok_or(ParsingError {
					etype: ParsingErrorType::InvalidOperands,
					line_no: token.line_no,
				})?;

				match (&op1.ttype, &op2.ttype) {
					(AssemblyTokenType::Number(imm), AssemblyTokenType::Register(reg)) => {
						result.push(Instruction::MoveImmToReg(*imm as u16, *reg));
					},

					_ => return Err(ParsingError {
						etype: ParsingErrorType::InvalidOperands,
						line_no: token.line_no,
					}),
				}
			},

			"halt" => {
				result.push(Instruction::Halt);
			},

			_ => {
				return Err(ParsingError {
					etype: ParsingErrorType::InvalidInstruction(ident),
					line_no: token.line_no,
				})
			},
		}
	}

	Ok(result)
}

/// The ATLAS PC virtual machine.
pub struct AtlasVM {
	program_counter: u16,
	pub registers: Vec<u16>,
	memory: Vec<u16>,
}

impl AtlasVM {
	/// Returns a new [`AtlasVM`].
	pub fn new() -> Self {
		Self {
			program_counter: 0x0000,
			registers: vec![0x0000; REGISTER_AMOUNT],
			memory: vec![0x0000; RAM_SIZE / 2],
		}
	}

	/// Reads a word of memory and returns the result.
	fn read_memory_word(&self, address: u16) -> u16 {
		self.memory[address as usize / 2]
	}

	/// Reads a byte of memory and returns the result.
	fn read_memory_byte(&self, address: u16) -> u8 {
		let word = self.memory[address as usize / 2];

		if address % 2 == 0 {
			(word >> 8) as u8
		} else {
			(word & 0xFF) as u8
		}
	}

	/// Writes a word to memory.
	fn write_memory_word(&mut self, address: u16, value: u16) {
		self.memory[address as usize / 2] = value;
	}

	/// Writes a byte to memory.
	fn write_memory_byte(&mut self, address: u16, value: u8) {
		let word = self.memory[address as usize / 2];

		let new_word: u16 = if address % 2 == 0 {
			((value as u16) << 8) | (word & 0xFF)
		} else {
			((word >> 8) << 8) | (value as u16)
		};

		self.memory[address as usize / 2] = new_word;
	}

	/// Runs an assembly program.
	pub fn run(&mut self, assembly: String) {
		let instrs = match parse_assembly(assembly) {
			Ok(instrs) => instrs,
			Err(err) => panic!("{}", err),
		};

		let machine_code = instrs.iter().flat_map(|instr| instr.as_machine_code());

		crate::log!("{}", machine_code.clone().map(|byte| format!("{:02x}", byte)).collect::<Vec<_>>().join(" "));

		for (addr, byte) in machine_code.enumerate() {
			self.write_memory_byte(addr as u16, byte);
		}

		let mut is_halted = false;

		while !is_halted {
			let instr_code = self.read_memory_byte(self.program_counter);

			match instr_code {
				// Halt
				0x00 => {
					is_halted = true;
				},

				// MoveImmToReg
				0x02 => {
					let reg = self.read_memory_byte(self.program_counter + 1);
					let imm = self.read_memory_word(self.program_counter + 2);

					self.registers[reg as usize] = imm;

					self.program_counter += 4;
				},

				_ => panic!("Invalid instruction code {:02x}", instr_code),
			}
		}
	}
}

impl Default for AtlasVM {
    fn default() -> Self {
        Self::new()
    }
}

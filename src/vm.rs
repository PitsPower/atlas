//! The ATLAS PC Virtual Machine and assembler. Used as a reference for the actual computer.

use std::collections::HashMap;

/// The number of registers.
const REGISTER_AMOUNT: usize = 16;
/// The size of RAM.
const RAM_SIZE: usize = 0x10000;	// Max for a 16-bit address

/// The different kinds of errors that can happen when assembling.
#[derive(Debug)]
enum AssembleErrorType {
	/// Invalid character.
	InvalidCharacter(char),
	/// Unexpected end of file.
	UnexpectedEOF,
	/// Unexpected token.
	UnexpectedToken(AssemblyTokenType),
	/// The instruction being executed isn't known.
	InvalidInstruction(String),
	/// The instruction has been given invalid operands.
	InvalidOperands,
	/// The label being referenced isn't defined.
	UndefinedLabel(String),
}

impl std::fmt::Display for AssembleErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidCharacter(char) => write!(f, "Unexpected character: '{}'", char),
			Self::UnexpectedEOF => write!(f, "Unexpected end of file"),
			Self::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
			Self::InvalidInstruction(instr) => write!(f, "Invalid instruction: {}", instr),
			Self::InvalidOperands => write!(f, "Invalid operands"),
			Self::UndefinedLabel(label) => write!(f, "Undefined label: {}", label),
		}
    }
}

/// An error that may occur when assembling. Includes the type of error as well as things
/// such as the line number.
struct AssembleError {
	etype: AssembleErrorType,
	line_no: usize,
}

impl std::fmt::Display for AssembleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
			f,
			"Error on line {}: {}",
			self.line_no,
			self.etype,
		)
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
			Self::Colon => write!(f, ":"),
			Self::Comma => write!(f, ","),
			Self::LSquare => write!(f, "["),
			Self::RSquare => write!(f, "]"),
			Self::Identifier(ident) => write!(f, "{}", ident),
			Self::Number(num) => write!(f, "{}", num),
			Self::Register(reg) => write!(f, "${}", reg),
			Self::String(str) => write!(f, "\"{}\"", str),
		}
    }
}

/// The set of ATLAS assembly instructions.
#[derive(Debug)]
enum InstructionType {
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

	// Temporary instructions that don't have any machine code counterpart.
	// Usually these are instructions that use labels directly instead of immediate addresses
	// or offsets.

	/// Moves the address represented by a label into a register.
	MoveLabelToReg(String, usize),
}

impl InstructionType {
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

			Self::MoveLabelToReg(_, _) => unreachable!(),
		}
	}

	/// Returns the size of the instruction in machine code.
	fn get_machine_code_size(&self) -> usize {
		match self {
			Self::MoveLabelToReg(_, _) => 4,
			_ => self.as_machine_code().len(),
		}
	}
}

/// An assembly instruction. Includes the type of instruction and an optional label.
struct Instruction {
	itype: InstructionType,
	label: Option<String>,
	line_no: usize,
}

/// Lexes an assembly program.
fn lex_assembly(assembly: String) -> Result<Vec<AssemblyToken>, AssembleError> {
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
							return Err(AssembleError { etype: AssembleErrorType::UnexpectedEOF, line_no });
						}
					} else if char != '"' {
						string.push(chars.next().unwrap());
					} else {
						break;
					}
				}

				if chars.next().is_none() {
					return Err(AssembleError { etype: AssembleErrorType::UnexpectedEOF, line_no });
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
						return Err(AssembleError { etype: AssembleErrorType::InvalidCharacter(char), line_no });
					},
				}

				chars.next().unwrap();
			},
		}
	}

	Ok(result)
}

/// An assembly operand.
enum AssemblyOperand {
	Immediate(u16),
	Register(usize),
	Label(String),
}

/// A parser for the ATLAS assembly language.
struct AssemblyParser {
	tokens: std::iter::Peekable<std::vec::IntoIter<AssemblyToken>>,
	eof_line_no: usize,
}

impl AssemblyParser {
	/// Returns a new [`AssemblyParser`].
	fn new(tokens: Vec<AssemblyToken>) -> Self {
		let eof_line_no = tokens.last().unwrap().line_no;

		Self {
			tokens: tokens.into_iter().peekable(),
			eof_line_no,
		}
	}

	/// Returns the next token while consuming it.
	fn eat_token(&mut self) -> Result<AssemblyToken, AssembleError> {
		self.tokens.next().ok_or(AssembleError {
			etype: AssembleErrorType::UnexpectedEOF,
			line_no: self.eof_line_no,
		})
	}

	/// Eats a token if it equals the input token, otherwise returns an error.
	fn eat_token_of_type(&mut self, ttype: AssemblyTokenType) -> Result<(), AssembleError> {
		let token = self.eat_token()?;

		if token.ttype == ttype {
			Ok(())
		} else {
			Err(AssembleError {
				etype: AssembleErrorType::UnexpectedToken(token.ttype),
				line_no: token.line_no,
			})
		}
	}

	/// Returns the line number of the current token.
	fn get_line_no(&mut self) -> Result<usize, AssembleError> {
		let token = self.tokens.peek().ok_or(AssembleError {
			etype: AssembleErrorType::UnexpectedEOF,
			line_no: self.eof_line_no,
		})?;
		Ok(token.line_no)
	}

	/// Returns the string of the next identifier if applicable, otherwise returns an error.
	fn eat_ident(&mut self) -> Result<String, AssembleError> {
		let token = self.eat_token()?;

		match &token.ttype {
			AssemblyTokenType::Identifier(ident) => Ok(ident.clone()),
			_ => Err(AssembleError {
				etype: AssembleErrorType::UnexpectedToken(token.ttype.clone()),
				line_no: token.line_no,
			}),
		}
	}

	/// Returns the next operand if applicable, otherwise returns an error.
	fn eat_operand(&mut self) -> Result<AssemblyOperand, AssembleError> {
		let token = self.eat_token()?;

		match &token.ttype {
			AssemblyTokenType::Number(num) => Ok(AssemblyOperand::Immediate(*num as u16)),
			AssemblyTokenType::Register(reg) => Ok(AssemblyOperand::Register(*reg)),
			AssemblyTokenType::Identifier(ident) => Ok(AssemblyOperand::Label(ident.clone())),
			
			_ => Err(AssembleError {
				etype: AssembleErrorType::UnexpectedToken(token.ttype.clone()),
				line_no: token.line_no,
			}),
		}
	}

	/// Parses the tokens and returns a list of instructions.
	fn parse(&mut self) -> Result<Vec<Instruction>, AssembleError> {
		let mut result = vec![];
	
		while self.tokens.peek().is_some() {
			let line_no = self.get_line_no()?;
			let ident = self.eat_ident()?;
	
			// The identifer is either an instruction name or a label name
			// We use the next token to determine which one it is

			let mut label = None;
			let instr_name;
	
			match self.tokens.peek() {
				Some(AssemblyToken { ttype: AssemblyTokenType::Colon, line_no: _ }) => {
					label = Some(ident.clone());
					self.tokens.next().unwrap();
					instr_name = self.eat_ident()?;
				},
				_ => {
					instr_name = ident.clone();
				},
			}
	
			match instr_name.as_str() {
				"mov" => {
					let op1 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op2 = self.eat_operand()?;
	
					match (&op1, &op2) {
						(AssemblyOperand::Immediate(imm), AssemblyOperand::Register(reg)) => {
							result.push(Instruction {
								itype: InstructionType::MoveImmToReg(*imm as u16, *reg),
								label,
								line_no,
							});
						},
						(AssemblyOperand::Label(label_str), AssemblyOperand::Register(reg)) => {
							result.push(Instruction {
								itype: InstructionType::MoveLabelToReg(label_str.to_string(), *reg),
								label,
								line_no,
							});
						},
	
						_ => return Err(AssembleError {
							etype: AssembleErrorType::InvalidOperands,
							line_no,
						}),
					}
				},
	
				"halt" => {
					result.push(Instruction {
						itype: InstructionType::Halt,
						label,
						line_no,
					});
				},
	
				_ => {
					return Err(AssembleError {
						etype: AssembleErrorType::InvalidInstruction(instr_name),
						line_no,
					})
				},
			}
		}
	
		Ok(result)
	}
}

/// Produces machine instructions given an assembly program.
fn assemble(assembly: String) -> Result<Vec<u8>, AssembleError> {
	// Convert assembly string to tokens
	let tokens = lex_assembly(assembly)?;

	// Convert tokens to parsed instructions
	let mut parser = AssemblyParser::new(tokens);
	let mut instrs = parser.parse()?;

	// Resolve labels to addresses

	let mut label_map: HashMap<String, usize> = HashMap::new();

	let mut current_address = 0;

	for instr in &mut instrs {
		if let Some(label) = &instr.label {
			label_map.insert(label.clone(), current_address);
		}

		current_address += instr.itype.get_machine_code_size();
	}

	for instr in &mut instrs {
		#[allow(clippy::single_match)]
		match &instr.itype {
			InstructionType::MoveLabelToReg(label, reg) => {
				if let Some(address) = label_map.get(label) {
					instr.itype = InstructionType::MoveImmToReg(*address as u16, *reg);
				} else {
					return Err(AssembleError {
						etype: AssembleErrorType::UndefinedLabel(label.clone()),
						line_no: instr.line_no,
					});
				}

			},
			
			_ => {},
		}
	}

	// Convert the resulting instructions into machine code and return
	Ok(instrs.iter().flat_map(|instr| instr.itype.as_machine_code()).collect())
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
		let machine_code = match assemble(assembly) {
			Ok(machine_code) => machine_code,
			Err(err) => panic!("{}", err),
		};

		crate::log!("{}", machine_code.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<_>>().join(" "));

		for (addr, byte) in machine_code.iter().enumerate() {
			self.write_memory_byte(addr as u16, *byte);
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

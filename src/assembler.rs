//! The AtlASM assembler.

use std::collections::HashMap;

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
			Self::InvalidCharacter(char) => write!(f, "Unexpected character: '{char}'"),
			Self::UnexpectedEOF => write!(f, "Unexpected end of file"),
			Self::UnexpectedToken(token) => write!(f, "Unexpected token: {token}"),
			Self::InvalidInstruction(instr) => write!(f, "Invalid instruction: {instr}"),
			Self::InvalidOperands => write!(f, "Invalid operands"),
			Self::UndefinedLabel(label) => write!(f, "Undefined label: {label}"),
		}
    }
}

/// An error that may occur when assembling. Includes the type of error as well as things
/// such as the line number.
#[derive(Debug)]
pub struct AssembleError {
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
#[derive(Debug)]
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
			Self::Identifier(ident) => write!(f, "{ident}"),
			Self::Number(num) => write!(f, "{num}"),
			Self::Register(reg) => write!(f, "${reg}"),
			Self::String(str) => write!(f, "\"{str}\""),
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
	/// Moves the data from the address stored in the first register to the address in the second register.
	MoveRegAddrToReg(usize, usize),
	/// Moves the data in the register to the address.
	// MoveRegToImmAddr(usize, u16),

	MoveByteRegToRegAddr(usize, usize),
	MoveByteRegAddrToReg(usize, usize),
	// MoveByteRegToImmAddr(usize, u16),

	/// Adds the first register and second register and stores the result in the third register.
	AddRegToReg(usize, usize, usize),
	/// Adds the immediate value to the first register and stores the result in the second register.
	AddImmToReg(usize, u16, usize),

	/// Subtracts the second register from the first register and stores the result in the third register.
	SubRegFromReg(usize, usize, usize),
	/// Subtracts the immediate value from the first register and stores the result in the second register.
	SubImmFromReg(usize, u16, usize),

	/// Branches to a given address.
	Branch(u16),
	/// Branches to a given address if the register's value is equal to the immediate.
	BranchIfEqual(usize, u16, u16),
	/// Branches to a given address if the register's value is less than or equal to the immediate.
	BranchIfLessThanOrEqual(usize, u16, u16),

	/// Jumps to an address and saves the next instruction address in $15.
	Call(u16),
	/// Jumps to the address in $15.
	Ret,

	// Assembly directives. Not exactly instructions but whatever.

	/// Writes some data to the binary output.
	DataDirective(Vec<u8>),

	// Temporary instructions that don't have any machine code counterpart.
	// Usually these are instructions that use labels directly instead of immediate addresses
	// or offsets.

	/// Moves the address represented by a label into a register.
	MoveLabelToReg(String, usize),

	/// Branch, but branches to a label.
	BranchToLabel(String),
	/// BranchIfEqual, but branches to a label.
	BranchToLabelIfEqual(usize, u16, String),
	/// BranchIfLessThanOrEqual, but branches to a label.
	BranchToLabelIfLessThanOrEqual(usize, u16, String),

	/// Call, but calls a label.
	CallLabel(String),
}

impl InstructionType {
	/// Returns the instruction as a machine code instruction.
	fn as_machine_code(&self) -> Vec<u8> {
		match self {
			Self::Halt => {
				vec![0x00, 0xff]
			},

			Self::MoveRegToReg(r1, r2) => {
				vec![0x01, 0xff, *r1 as u8, *r2 as u8]
			},
			Self::MoveImmToReg(imm, reg) => {
				let immb = imm.to_be_bytes();
				vec![0x02, *reg as u8, immb[0], immb[1]]
			},
			Self::MoveRegToRegAddr(r1, r2) => {
				vec![0x03, 0xff, *r1 as u8, *r2 as u8]
			},
			Self::MoveRegAddrToReg(r1, r2) => {
				vec![0x04, 0xff, *r1 as u8, *r2 as u8]
			},
			// Self::MoveRegToImmAddr(reg, imm) => {
			// 	let immb = imm.to_be_bytes();
			// 	vec![0x05, *reg as u8, immb[0], immb[1]]
			// },

			Self::AddRegToReg(r1, r2, r3) => {
				vec![0x06, *r1 as u8, *r2 as u8, *r3 as u8]
			},
			Self::AddImmToReg(r1, imm, r2) => {
				let immb = imm.to_be_bytes();
				vec![0x07, 0xff, *r1 as u8, *r2 as u8, immb[0], immb[1]]
			},

			Self::SubRegFromReg(r1, r2, r3) => {
				vec![0x26, *r1 as u8, *r2 as u8, *r3 as u8]
			},
			Self::SubImmFromReg(r1, imm, r2) => {
				let immb = imm.to_be_bytes();
				vec![0x27, 0xff, *r1 as u8, *r2 as u8, immb[0], immb[1]]
			},

			Self::Branch(addr) => {
				let addrb = addr.to_be_bytes();
				vec![0x08, 0xff, addrb[0], addrb[1]]
			},
			Self::BranchIfEqual(reg, imm, addr) => {
				let immb = imm.to_be_bytes();
				let addrb = addr.to_be_bytes();
				vec![0x09, *reg as u8, immb[0], immb[1], addrb[0], addrb[1]]
			},
			Self::BranchIfLessThanOrEqual(reg, imm, addr) => {
				let immb = imm.to_be_bytes();
				let addrb = addr.to_be_bytes();
				vec![0x0a, *reg as u8, immb[0], immb[1], addrb[0], addrb[1]]
			},
			
			Self::MoveByteRegToRegAddr(r1, r2) => {
				vec![0x13, 0xff, *r1 as u8, *r2 as u8]
			},
			Self::MoveByteRegAddrToReg(r1, r2) => {
				vec![0x14, 0xff, *r1 as u8, *r2 as u8]
			},
			// Self::MoveByteRegToImmAddr(reg, imm) => {
			// 	let immb = imm.to_be_bytes();
			// 	vec![0x15, *reg as u8, immb[0], immb[1]]
			// },

			Self::Call(addr) => {
				let addrb = addr.to_be_bytes();
				vec![0x16, 0x0f, addrb[0], addrb[1]]
			},
			Self::Ret => {
				vec![0x17, 0x0f]
			},

			Self::DataDirective(data) => data.clone(),

			_ => unreachable!(),
		}
	}

	/// Returns the size of the instruction in machine code.
	fn get_machine_code_size(&self) -> usize {
		match self {
			Self::MoveLabelToReg(_, _) => 4,
			Self::BranchToLabel(_) => 4,
			Self::BranchToLabelIfEqual(_, _, _) => 6,
			Self::BranchToLabelIfLessThanOrEqual(_, _, _) => 6,
			Self::CallLabel(_) => 4,
			_ => self.as_machine_code().len(),
		}
	}
}

/// An assembly instruction. Includes the type of instruction and an optional label.
#[derive(Debug)]
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
				identifier.push(chars.next().unwrap());

				while let Some(&char) = chars.peek() {
					if char.is_alphanumeric() || char == '.' || char == '_' {
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
					' ' | '\t' | '\r' => {},

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

/// An assembly operand type.
enum AssemblyOperandType {
	Immediate(u16),
	ImmediateAddress(u16),
	Label(String),
	Register(usize),
	RegisterAddress(usize),
	String(String),
}

/// An assembly operand.
struct AssemblyOperand {
	otype: AssemblyOperandType,
	line_no: usize,
}

impl AssemblyOperand {
	fn as_bytes(&self) -> Result<Vec<u8>, AssembleError> {
		match &self.otype {
			AssemblyOperandType::Immediate(imm) => {
				Ok(vec![*imm as u8])
			},
			AssemblyOperandType::String(str) => {
				Ok(str.as_bytes().to_vec())
			},

			_ => {
				Err(AssembleError {
					etype: AssembleErrorType::InvalidOperands,
					line_no: self.line_no,
				})
			},
		}
	}
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

	/// Returns the line number of the current token, or the end of file line if no tokens
	/// are left.
	fn get_line_no(&mut self) -> usize {
		if let Some(token) = self.tokens.peek() {
			token.line_no
		} else {
			self.eof_line_no
		}
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
			AssemblyTokenType::Number(num) => Ok(AssemblyOperand {
				otype: AssemblyOperandType::Immediate(*num as u16),
				line_no: token.line_no,
			}),
			AssemblyTokenType::Identifier(ident) => Ok(AssemblyOperand {
				otype: AssemblyOperandType::Label(ident.clone()),
				line_no: token.line_no,
			}),
			AssemblyTokenType::Register(reg) => Ok(AssemblyOperand {
				otype: AssemblyOperandType::Register(*reg),
				line_no: token.line_no,
			}),
			AssemblyTokenType::String(str) => Ok(AssemblyOperand {
				otype: AssemblyOperandType::String(str.clone()),
				line_no: token.line_no,
			}),
			
			AssemblyTokenType::LSquare => {
				let operand = self.eat_operand()?;
				self.eat_token_of_type(AssemblyTokenType::RSquare)?;

				match operand.otype {
					AssemblyOperandType::Immediate(imm) => Ok(AssemblyOperand {
						otype: AssemblyOperandType::ImmediateAddress(imm),
						line_no: token.line_no,
					}),
					AssemblyOperandType::Register(reg) => Ok(AssemblyOperand {
						otype: AssemblyOperandType::RegisterAddress(reg),
						line_no: token.line_no,
					}),

					_ => Err(AssembleError {
						etype: AssembleErrorType::InvalidOperands,
						line_no: token.line_no,
					}),
				}
			}
			
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
			let ident = self.eat_ident()?;
	
			// The identifer is either an instruction name or a label name
			// We use the next token to determine which one it is

			let mut label = None;
			let line_no;
			let instr_name;

			if let Some(AssemblyToken { ttype: AssemblyTokenType::Colon, line_no: _ }) = self.tokens.peek() {
				label = Some(ident.clone());
				self.tokens.next().unwrap();
				line_no = self.get_line_no();
				instr_name = self.eat_ident()?;
			} else {
				line_no = self.get_line_no();
				instr_name = ident.clone();
			}
	
			match instr_name.as_str() {
				"mov" => {
					let op1 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op2 = self.eat_operand()?;
	
					match (op1.otype, op2.otype) {
						(AssemblyOperandType::Immediate(imm), AssemblyOperandType::Register(reg)) => {
							result.push(Instruction {
								itype: InstructionType::MoveImmToReg(imm, reg),
								label,
								line_no,
							});
						},
						(AssemblyOperandType::Label(label_str), AssemblyOperandType::Register(reg)) => {
							result.push(Instruction {
								itype: InstructionType::MoveLabelToReg(label_str.to_string(), reg),
								label,
								line_no,
							});
						},
						(AssemblyOperandType::Register(r1), AssemblyOperandType::Register(r2)) => {
							result.push(Instruction {
								itype: InstructionType::MoveRegToReg(r1, r2),
								label,
								line_no,
							});
						},
						(AssemblyOperandType::RegisterAddress(r1), AssemblyOperandType::Register(r2)) => {
							result.push(Instruction {
								itype: InstructionType::MoveRegAddrToReg(r1, r2),
								label,
								line_no,
							});
						},
						(AssemblyOperandType::Register(r1), AssemblyOperandType::RegisterAddress(r2)) => {
							result.push(Instruction {
								itype: InstructionType::MoveRegToRegAddr(r1, r2),
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

				"movb" => {
					let op1 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op2 = self.eat_operand()?;
	
					match (op1.otype, op2.otype) {
						(AssemblyOperandType::RegisterAddress(r1), AssemblyOperandType::Register(r2)) => {
							result.push(Instruction {
								itype: InstructionType::MoveByteRegAddrToReg(r1, r2),
								label,
								line_no,
							});
						},
						(AssemblyOperandType::Register(r1), AssemblyOperandType::RegisterAddress(r2)) => {
							result.push(Instruction {
								itype: InstructionType::MoveByteRegToRegAddr(r1, r2),
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

				"add" => {
					let op1 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op2 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op3 = self.eat_operand()?;

					match (op1.otype, op2.otype, op3.otype) {
						(
							AssemblyOperandType::Register(r1),
							AssemblyOperandType::Immediate(imm),
							AssemblyOperandType::Register(r2),
						) => {
							result.push(Instruction {
								itype: InstructionType::AddImmToReg(r1, imm, r2),
								label,
								line_no,
							});
						},

						(
							AssemblyOperandType::Register(r1),
							AssemblyOperandType::Register(r2),
							AssemblyOperandType::Register(r3),
						) => {
							result.push(Instruction {
								itype: InstructionType::AddRegToReg(r1, r2, r3),
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
				"sub" => {
					let op1 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op2 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op3 = self.eat_operand()?;

					match (op1.otype, op2.otype, op3.otype) {
						(
							AssemblyOperandType::Register(r1),
							AssemblyOperandType::Immediate(imm),
							AssemblyOperandType::Register(r2),
						) => {
							result.push(Instruction {
								itype: InstructionType::SubImmFromReg(r1, imm, r2),
								label,
								line_no,
							});
						},

						(
							AssemblyOperandType::Register(r1),
							AssemblyOperandType::Register(r2),
							AssemblyOperandType::Register(r3),
						) => {
							result.push(Instruction {
								itype: InstructionType::SubRegFromReg(r1, r2, r3),
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

				"b" => {
					let op = self.eat_operand()?;

					if let AssemblyOperandType::Label(label_str) = op.otype {
						result.push(Instruction {
							itype: InstructionType::BranchToLabel(label_str),
							label,
							line_no,
						});
					} else {
						return Err(AssembleError {
							etype: AssembleErrorType::InvalidOperands,
							line_no,
						});
					}
				},

				"beq" => {
					let op1 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op2 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op3 = self.eat_operand()?;

					match (op1.otype, op2.otype, op3.otype) {
						(
							AssemblyOperandType::Register(reg),
							AssemblyOperandType::Immediate(imm),
							AssemblyOperandType::Label(label_str),
						) => {
							result.push(Instruction {
								itype: InstructionType::BranchToLabelIfEqual(reg, imm, label_str),
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

				"bleq" => {
					let op1 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op2 = self.eat_operand()?;
					self.eat_token_of_type(AssemblyTokenType::Comma)?;
					let op3 = self.eat_operand()?;

					match (op1.otype, op2.otype, op3.otype) {
						(
							AssemblyOperandType::Register(reg),
							AssemblyOperandType::Immediate(imm),
							AssemblyOperandType::Label(label_str),
						) => {
							result.push(Instruction {
								itype: InstructionType::BranchToLabelIfLessThanOrEqual(reg, imm, label_str),
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

				"call" => {
					let op = self.eat_operand()?;

					if let AssemblyOperandType::Label(label_str) = op.otype {
						result.push(Instruction {
							itype: InstructionType::CallLabel(label_str),
							label,
							line_no,
						});
					} else {
						return Err(AssembleError {
							etype: AssembleErrorType::InvalidOperands,
							line_no,
						});
					}
				},
				"ret" => {
					result.push(Instruction {
						itype: InstructionType::Ret,
						label,
						line_no,
					});
				},
	
				".db" => {
					let mut bytes = vec![];

					let operand = self.eat_operand()?;
					bytes.append(&mut operand.as_bytes()?);

					while let Some(AssemblyToken { ttype: AssemblyTokenType::Comma, line_no: _ }) = self.tokens.peek() {
						self.eat_token()?;
						let operand = self.eat_operand()?;
						bytes.append(&mut operand.as_bytes()?);
					}

					result.push(Instruction {
						itype: InstructionType::DataDirective(bytes),
						label,
						line_no,
					});
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
					});
				},
			}
		}
	
		Ok(result)
	}
}

/// Produces machine instructions given an assembly program.
pub fn assemble(assembly: String) -> Result<Vec<u8>, AssembleError> {
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
			
			InstructionType::BranchToLabel(label) => {
				if let Some(address) = label_map.get(label) {
					instr.itype = InstructionType::Branch(*address as u16);
				} else {
					return Err(AssembleError {
						etype: AssembleErrorType::UndefinedLabel(label.clone()),
						line_no: instr.line_no,
					});
				}
			},
			
			InstructionType::BranchToLabelIfEqual(reg, imm, label) => {
				if let Some(address) = label_map.get(label) {
					instr.itype = InstructionType::BranchIfEqual(*reg, *imm, *address as u16);
				} else {
					return Err(AssembleError {
						etype: AssembleErrorType::UndefinedLabel(label.clone()),
						line_no: instr.line_no,
					});
				}
			},
			
			InstructionType::BranchToLabelIfLessThanOrEqual(reg, imm, label) => {
				if let Some(address) = label_map.get(label) {
					instr.itype = InstructionType::BranchIfLessThanOrEqual(*reg, *imm, *address as u16);
				} else {
					return Err(AssembleError {
						etype: AssembleErrorType::UndefinedLabel(label.clone()),
						line_no: instr.line_no,
					});
				}
			},
			
			InstructionType::CallLabel(label) => {
				if let Some(address) = label_map.get(label) {
					instr.itype = InstructionType::Call(*address as u16);
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

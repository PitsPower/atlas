//! A collection of ATLAS virtual machines. Used as a reference for the actual computer.

use std::convert::TryFrom;

use crate::assembler::*;

/// The number of registers.
const REGISTER_AMOUNT: usize = 16;
/// The size of RAM.
const RAM_SIZE: usize = 0x10000;	// Max for a 16-bit address

/// Virtual implementation of memory.
pub struct Memory {
	data: Vec<u16>,
}

impl Memory {
	/// Returns a new [`Memory`].
	fn new() -> Self {
		Self {
			data: vec![0x0000; RAM_SIZE / 2],
		}
	}

	/// Reads a word of memory and returns the result.
	fn read_memory_word(&self, address: u16) -> u16 {
		self.data[address as usize / 2]
	}

	/// Reads a byte of memory and returns the result.
	fn read_memory_byte(&self, address: u16) -> u8 {
		let word = self.data[address as usize / 2];

		if address % 2 == 0 {
			(word >> 8) as u8
		} else {
			(word & 0xFF) as u8
		}
	}

	/// Writes a word to memory.
	fn write_memory_word(&mut self, address: u16, value: u16) {
		self.data[address as usize / 2] = value;
	}

	/// Writes a byte to memory.
	fn write_memory_byte(&mut self, address: u16, value: u8) {
		let word = self.data[address as usize / 2];

		let new_word: u16 = if address % 2 == 0 {
			((value as u16) << 8) | (word & 0xFF)
		} else {
			((word >> 8) << 8) | (value as u16)
		};

		self.data[address as usize / 2] = new_word;
	}

	/// Returns the screen contents as a string.
	pub fn read_screen(&self) -> String {
		let screen_addr = 0xfc00;

		let screen_width = 64;
		let screen_height = 16;

		let mut result = String::new();

		for i in 0..screen_height {
			for j in 0..screen_width {
				let addr = screen_addr + i * screen_width + j;
				result.push(self.read_memory_byte(addr) as char);
			}
			result.push('\n');
		}

		result
	}
}

/// The highest level ATLAS PC virtual machine. Used as a reference for all the rest.
pub struct AtlasVM {
	pub registers: [u16; REGISTER_AMOUNT],
	program_counter: u16,
	pub memory: Memory,
}

impl AtlasVM {
	/// Returns a new [`AtlasVM`].
	pub fn new() -> Self {
		Self {
			registers: [0x0000; REGISTER_AMOUNT],
			program_counter: 0x0000,
			memory: Memory::new(),
		}
	}

	/// Runs an assembly program.
	pub fn run(&mut self, assembly: String) {
		let machine_code = match assemble(assembly) {
			Ok(machine_code) => machine_code,
			Err(err) => panic!("{}", err),
		};

		crate::log!("ASSEMBLED PROGRAM:\n\n{}", machine_code.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join(" "));

		for (addr, byte) in machine_code.iter().enumerate() {
			self.memory.write_memory_byte(addr as u16, *byte);
		}

		let mut is_halted = false;

		while !is_halted {
			let instr_code = self.memory.read_memory_byte(self.program_counter);

			match instr_code {
				// Halt
				0x00 => {
					is_halted = true;
				},

				// MovRegToReg
				0x01 => {
					let r1 = self.memory.read_memory_byte(self.program_counter + 2);
					let r2 = self.memory.read_memory_byte(self.program_counter + 3);

					self.registers[r2 as usize] = self.registers[r1 as usize];

					self.program_counter += 4;
				},

				// MoveImmToReg
				0x02 => {
					let reg = self.memory.read_memory_byte(self.program_counter + 1);
					let imm = self.memory.read_memory_word(self.program_counter + 2);

					self.registers[reg as usize] = imm;

					self.program_counter += 4;
				},

				// MoveRegToRegAddr
				0x03 => {
					let r1 = self.memory.read_memory_byte(self.program_counter + 2);
					let r2 = self.memory.read_memory_byte(self.program_counter + 3);

					self.memory.write_memory_word(self.registers[r2 as usize], self.registers[r1 as usize]);

					self.program_counter += 4;
				},

				// MoveRegAddrToReg
				0x04 => {
					let r1 = self.memory.read_memory_byte(self.program_counter + 2);
					let r2 = self.memory.read_memory_byte(self.program_counter + 3);

					self.registers[r2 as usize] = self.memory.read_memory_word(self.registers[r1 as usize]);

					self.program_counter += 4;
				},

				// AddRegToReg
				0x06 => {
					let r1 = self.memory.read_memory_byte(self.program_counter + 1);
					let r2 = self.memory.read_memory_byte(self.program_counter + 2);
					let r3 = self.memory.read_memory_byte(self.program_counter + 3);

					self.registers[r3 as usize] = self.registers[r1 as usize].wrapping_add(self.registers[r2 as usize]);
					
					self.program_counter += 4;
				},

				// AddImmToReg
				0x07 => {
					let r1 = self.memory.read_memory_byte(self.program_counter + 2);
					let imm = self.memory.read_memory_word(self.program_counter + 4);
					let r2 = self.memory.read_memory_byte(self.program_counter + 3);

					self.registers[r2 as usize] = self.registers[r1 as usize].wrapping_add(imm);
					
					self.program_counter += 6;
				},

				// Branch
				0x08 => {
					let addr = self.memory.read_memory_word(self.program_counter + 2);
					self.program_counter = addr;
				},

				// BranchIfEqual
				0x09 => {
					let reg = self.memory.read_memory_byte(self.program_counter + 1);
					let imm = self.memory.read_memory_word(self.program_counter + 2);
					let addr = self.memory.read_memory_word(self.program_counter + 4);

					if self.registers[reg as usize] == imm {
						self.program_counter = addr;
					} else {
						self.program_counter += 6;
					}
				},

				// BranchIfLessThanOrEqual
				0x0a => {
					let reg = self.memory.read_memory_byte(self.program_counter + 1);
					let imm = self.memory.read_memory_word(self.program_counter + 2);
					let addr = self.memory.read_memory_word(self.program_counter + 4);

					if self.registers[reg as usize] <= imm {
						self.program_counter = addr;
					} else {
						self.program_counter += 6;
					}
				},

				// MoveByteRegToRegAddr
				0x13 => {
					let r1 = self.memory.read_memory_byte(self.program_counter + 2);
					let r2 = self.memory.read_memory_byte(self.program_counter + 3);

					self.memory.write_memory_byte(self.registers[r2 as usize], (self.registers[r1 as usize] & 0xff) as u8);

					self.program_counter += 4;
				},

				// MoveByteRegAddrToReg
				0x14 => {
					let r1 = self.memory.read_memory_byte(self.program_counter + 2);
					let r2 = self.memory.read_memory_byte(self.program_counter + 3);

					self.registers[r2 as usize] = self.memory.read_memory_byte(self.registers[r1 as usize]) as u16;

					self.program_counter += 4;
				},

				_ => panic!("Invalid opcode: {:02x}", instr_code),
			}
		}
	}
}

impl Default for AtlasVM {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
enum ControlRegister {
	Gpr1 = 0,
	Gpr2 = 1,
	Gpr3 = 2,
	Pc = 3,
	PcPlusTwoN = 4,
	Mar = 5,
	Mdr = 6,
	Ir1 = 7,
	Ir2 = 8,
	AluA = 9,
	AluB = 10,
	AluO = 11,
	BrAddr = 12,
	Branch = 13,
}

impl std::convert::TryFrom<u8> for ControlRegister {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
			0 =>  Ok(ControlRegister::Gpr1),
			1 =>  Ok(ControlRegister::Gpr2),
			2 =>  Ok(ControlRegister::Gpr3),
			3 =>  Ok(ControlRegister::Pc),
			4 =>  Ok(ControlRegister::PcPlusTwoN),
			5 =>  Ok(ControlRegister::Mar),
			6 =>  Ok(ControlRegister::Mdr),
			7 =>  Ok(ControlRegister::Ir1),
			8 =>  Ok(ControlRegister::Ir2),
			9 =>  Ok(ControlRegister::AluA),
			10 =>  Ok(ControlRegister::AluB),
			11 => Ok(ControlRegister::AluO),
			12 => Ok(ControlRegister::BrAddr),
			13 => Ok(ControlRegister::Branch),
			_ => Err(()),
		}
    }
}

#[derive(Clone, Copy)]
enum ControlFunc {
	Plus = 0,
	Minus = 1,
}

impl std::convert::TryFrom<u8> for ControlFunc {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
			0 => Ok(ControlFunc::Plus),
			1 => Ok(ControlFunc::Minus),
			_ => Err(()),
		}
    }
}

#[derive(Clone, Copy)]
struct ControlRomOutput {
	from: ControlRegister,
	to: ControlRegister,
	func: ControlFunc,
	n: usize,
	reset: bool,
}

impl ControlRomOutput {
	fn as_bytes(&self) -> u16 {
		(self.from as u16) << 12 | (self.to as u16) << 8 | (self.func as u16) << 3 | (self.n as u16) << 1 | self.reset as u16
	}
}

const CONTROL_ROM_MAX_STEPS: usize = 16;

macro_rules! cntrl {
	($from:ident => $to:ident) => {
		ControlRomOutput {
			from: ControlRegister::$from,
			to: ControlRegister::$to,
			func: ControlFunc::Plus,
			n: 0,
			reset: false,
		}
	};
	($from:ident => $to:ident, $op:ident) => {
		ControlRomOutput {
			from: ControlRegister::$from,
			to: ControlRegister::$to,
			func: ControlFunc::$op,
			n: 0,
			reset: false,
		}
	};
	(Pc + $twon:expr => $to:ident) => {
		ControlRomOutput {
			from: ControlRegister::PcPlusTwoN,
			to: ControlRegister::$to,
			func: ControlFunc::Plus,
			n: $twon / 2,
			reset: false,
		}
	};
}

fn add_steps_to_rom_data(data: &mut [u16; 256 * CONTROL_ROM_MAX_STEPS], opcode: u8, steps: Vec<ControlRomOutput>) {
	// Check that the step vector isn't too big
	if steps.len() + 2 > CONTROL_ROM_MAX_STEPS {
		panic!("Too many steps for opcode {:02x}", opcode);
	}

	// These two steps are done for all instructions
	data[opcode as usize * CONTROL_ROM_MAX_STEPS] = cntrl!(Pc => Mar).as_bytes();
	data[opcode as usize * CONTROL_ROM_MAX_STEPS + 1] = cntrl!(Mdr => Ir1).as_bytes();

	for (idx, step) in steps.iter().enumerate() {
		// If it's the last step, set the "reset" flag
		// to reset the control unit counter

		let mut edited_step = *step;

		if idx == steps.len() - 1 {
			edited_step.reset = true;
		}

		data[opcode as usize * CONTROL_ROM_MAX_STEPS + idx + 2] = edited_step.as_bytes();
	}
}

pub fn generate_control_rom_data() -> [u16; 256 * CONTROL_ROM_MAX_STEPS] {
	let mut result = [0; 256 * CONTROL_ROM_MAX_STEPS];

	// Halt
	add_steps_to_rom_data(&mut result, 0x00, vec![
		cntrl!(Pc => Pc),
	]);
	
	// MoveRegToReg
	add_steps_to_rom_data(&mut result, 0x01, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Ir2),
		cntrl!(Gpr2 => Gpr3),
		cntrl!(Pc+4 => Pc),
	]);
	
	// MoveImmToReg
	add_steps_to_rom_data(&mut result, 0x02, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Gpr1),
		cntrl!(Pc+4 => Pc),
	]);

	// MoveRegToRegAddr
	add_steps_to_rom_data(&mut result, 0x03, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Ir2),
		cntrl!(Gpr3 => Mar),
		cntrl!(Gpr2 => Mdr),
		cntrl!(Pc+4 => Pc),
	]);
	
	// MoveRegAddrToReg
	add_steps_to_rom_data(&mut result, 0x04, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Ir2),
		cntrl!(Gpr2 => Mar),
		cntrl!(Mdr => Gpr3),
		cntrl!(Pc+4 => Pc),
	]);
	
	// AddRegToReg
	add_steps_to_rom_data(&mut result, 0x06, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Ir2),
		cntrl!(Gpr1 => AluA),
		cntrl!(Gpr2 => AluB),
		cntrl!(AluO => Gpr3),
		cntrl!(Pc+4 => Pc),
	]);
	
	// AddImmToReg
	add_steps_to_rom_data(&mut result, 0x07, vec![
		cntrl!(Pc+4 => Mar),
		cntrl!(Mdr => AluB),
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Ir2),
		cntrl!(Gpr2 => AluA),
		cntrl!(AluO => Gpr3),
		cntrl!(Pc+6 => Pc),
	]);

	// Branch
	add_steps_to_rom_data(&mut result, 0x08, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Pc),
	]);
	
	// BranchIfEqual
	add_steps_to_rom_data(&mut result, 0x09, vec![
		cntrl!(Gpr1 => AluA),
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => AluB),
		cntrl!(Pc+4 => Mar),
		cntrl!(Mdr => BrAddr),
		cntrl!(Pc+6 => Pc),
		cntrl!(Branch => Pc, Minus),
	]);

	// MoveByteRegToRegAddr
	add_steps_to_rom_data(&mut result, 0x13, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Ir2),
		cntrl!(Gpr3 => Mar),
		cntrl!(Gpr2 => Mdr, Minus),
		cntrl!(Pc+4 => Pc),
	]);
	
	// MoveByteRegAddrToReg
	add_steps_to_rom_data(&mut result, 0x14, vec![
		cntrl!(Pc+2 => Mar),
		cntrl!(Mdr => Ir2),
		cntrl!(Gpr2 => Mar),
		cntrl!(Mdr => Gpr3, Minus),
		cntrl!(Pc+4 => Pc),
	]);

	result
}

/// A lower level virtual machine. This machine simulates the individual register moves
/// for each instruction and is controlled by a control unit ROM.
pub struct LowLevelAtlasVM {
	pub registers: [u16; REGISTER_AMOUNT],

	program_counter: u16,
	instruction_register_1: u16,
	instruction_register_2: u16,

	alu_register_a: u16,
	alu_register_b: u16,
	
	branch_address_register: u16,

	pub memory: Memory,
	memory_address_register: u16,

	control_rom: [u16; 256 * CONTROL_ROM_MAX_STEPS],
	control_counter: u8,
}

impl LowLevelAtlasVM {
	pub fn new() -> Self {
		Self {
			registers: [0x0000; REGISTER_AMOUNT],

			program_counter: 0x0000,
			instruction_register_1: 0x0000,
			instruction_register_2: 0x0000,
			
			alu_register_a: 0x0000,
			alu_register_b: 0x0000,
			
			branch_address_register: 0x0000,

			memory: Memory::new(),
			memory_address_register: 0x0000,

			control_rom: generate_control_rom_data(),
			control_counter: 0,
		}
	}

	fn read_register(&self, idx: u8, func: u8, n: u8) -> Option<u16> {
		if let Ok(reg) = ControlRegister::try_from(idx) {
			Some(match reg {
				ControlRegister::Gpr1 => self.registers[(self.instruction_register_1 & 0x0f) as usize],
				ControlRegister::Gpr2 => self.registers[(self.instruction_register_2 >> 8) as usize],
				ControlRegister::Gpr3 => self.registers[(self.instruction_register_2 & 0x0f) as usize],
				
				ControlRegister::Pc => self.program_counter,
				ControlRegister::PcPlusTwoN => self.program_counter + (n as u16) * 2,
				
				ControlRegister::Mar => self.memory_address_register,
				ControlRegister::Mdr => self.memory.read_memory_word(self.memory_address_register),
				ControlRegister::Ir1 => self.instruction_register_1,
				ControlRegister::Ir2 => self.instruction_register_2,
				
				ControlRegister::AluA => self.alu_register_a,
				ControlRegister::AluB => self.alu_register_b,
				ControlRegister::AluO => {
					match ControlFunc::try_from(func).unwrap() {
						ControlFunc::Plus => self.alu_register_a.wrapping_add(self.alu_register_b),
						ControlFunc::Minus => self.alu_register_a.wrapping_sub(self.alu_register_b),
					}
				},
				
				ControlRegister::Branch => {
					if self.alu_register_a == self.alu_register_b {
						self.branch_address_register
					} else {
						self.program_counter
					}
				},
				ControlRegister::BrAddr => self.branch_address_register,
			})
		} else {
			None
		}
	}

	fn write_register(&mut self, idx: u8, value: u16) -> bool {
		if let Ok(reg) = ControlRegister::try_from(idx) {
			match reg {
				ControlRegister::Gpr1 => {
					let reg = (self.instruction_register_1 & 0x0f) as usize;
					self.registers[reg] = value;
				},
				ControlRegister::Gpr2 => {
					let reg = (self.instruction_register_2 >> 8) as usize;
					self.registers[reg] = value;
				},
				ControlRegister::Gpr3 => {
					let reg = (self.instruction_register_2 & 0x0f) as usize;
					self.registers[reg] = value;
				},
				
				ControlRegister::Pc => self.program_counter = value,
				ControlRegister::PcPlusTwoN => todo!(),
				
				ControlRegister::Mar => self.memory_address_register = value,
				ControlRegister::Mdr => self.memory.write_memory_word(self.memory_address_register, value),
				ControlRegister::Ir1 => self.instruction_register_1 = value,
				ControlRegister::Ir2 => self.instruction_register_2 = value,
				
				ControlRegister::AluA => self.alu_register_a = value,
				ControlRegister::AluB => self.alu_register_b = value,
				ControlRegister::AluO => todo!(),
				
				ControlRegister::Branch => panic!("Write to ControlRegister::Branch"),
				ControlRegister::BrAddr => self.branch_address_register = value,
			};

			true
		} else {
			false
		}
	}

	/// Runs an assembly program.
	pub fn run(&mut self, assembly: String) {
		let machine_code = match assemble(assembly) {
			Ok(machine_code) => machine_code,
			Err(err) => panic!("{}", err),
		};

		crate::log!("ASSEMBLED PROGRAM:\n\n{}", machine_code.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join(" "));

		for (addr, byte) in machine_code.iter().enumerate() {
			self.memory.write_memory_byte(addr as u16, *byte);
		}
		
		let mut is_halted = false;

		while !is_halted {
			let opcode = self.instruction_register_1 >> 8;
			let control_rom_address = opcode << 4 | self.control_counter as u16;
			let control_word = self.control_rom[control_rom_address as usize];

			let from_reg = ((control_word >> 12) & 0x0f) as u8;
			let to_reg = ((control_word >> 8) & 0x0f) as u8;
			let func = ((control_word >> 3) & 0x1f) as u8;
			let n = ((control_word >> 1) & 0x03) as u8;
			let reset = (control_word & 0x01) as u8;

			let reg_value = self.read_register(from_reg, func, n)
				.unwrap_or_else(|| panic!("Read from unknown register index: {}", from_reg));

			if !self.write_register(to_reg, reg_value) {
				panic!("Write to unknown register index: {}", to_reg);
			}

			self.control_counter += 1;
			if reset == 1 {
				self.control_counter = 0;
			}

			// The only instruction that writes a register's value to itself is the "halt" instruction
			// In the real computer this just causes an infinite loop, but here we can actually halt
			is_halted = from_reg == to_reg;
		}
	}
}

impl Default for LowLevelAtlasVM {
    fn default() -> Self {
		Self::new()
	}
}

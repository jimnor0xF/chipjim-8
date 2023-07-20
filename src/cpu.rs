use crate::display::Display;

pub struct CPU {
    register: [u8; 16],
    memory: [u8; 4096],
    index: usize,
    pc: usize,
    stack: [u16; 16],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    display: [u8; 64 * 32],
    draw_flag: bool,
    running: bool,
    screen: &Display,
}

impl CPU {
    pub fn new(screen: &Display) -> CPU {
        CPU {
            register: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            display: [0; 64 * 32],
            draw_flag: false,
            running: true,
            screen: screen,
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let high_byte = self.memory[self.pc] as u16;
        let low_byte = self.memory[self.pc + 1] as u16;
        let opcode = (high_byte << 8) | low_byte;

        self.pc += 2;
        opcode
    }

    fn execute_opcode(&mut self, opcode: u16) -> () {
        // Mask off the first nibble to obtain op_type
        let op_type = opcode & 0xF000;

        // Mask off the second nibble to obtain X
        let x = (opcode & 0x0F00) >> 8;

        // Mask off the third nibble to obtain Y
        let y = (opcode & 0x00F0) >> 4;

        // Mask off the fourth nibble to obtain N
        let n = opcode & 0x000F;

        // Mask off the third and fourth nibbles to obtain NN
        let nn = opcode & 0x00FF;

        // Mask off the second, third, and fourth nibbles to obtain NNN
        let nnn = opcode & 0x0FFF;

        match op_type {
            0x0 => {
                if opcode == 0x00E0 {
                    println!("Clearing screen")
                }
            }
            0x1000 => {
                println!("Jump");
                self.pc = nnn as usize
            }
            0x6000 => {
                println!("Set 6XNN");
                self.register[x as usize] = nn as u8
            }
            0x7000 => {
                println!("Add value to register VX")
            }
            0xA000 => {
                println!("Set index register I")
            }
            0xD000 => {
                println!("Display/draw")
            }

            _ => println!("Not implemented yet"),
        }

        println!("Optype: 0x{:X}", op_type);
        println!("Opcode: 0x{:X}", opcode);
        println!("X: 0x{:X}", x);
        println!("Y: 0x{:X}", y);
        println!("N: 0x{:X}", n);
        println!("NN: 0x{:X}", nn);
        println!("NNN: 0x{:X}", nnn);
    }

    pub fn emulate_cycle(&mut self) -> bool {
        let opcode = self.fetch_opcode();
        if opcode == 0x0 {
            return false;
        }
        self.execute_opcode(opcode);
        true
    }
}

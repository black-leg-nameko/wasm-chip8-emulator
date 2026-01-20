use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pub pc: u16,
    stack: [u16; 16],
    sp: u16,
    display: [bool; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
}

#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8 {
        let mut mem = [0; 4096];
        
        // Sprite data (0x300)
        mem[0x300] = 0xF0; mem[0x301] = 0x90; mem[0x302] = 0x90; mem[0x303] = 0xF0;

        // Program (0x200)
        mem[0x200] = 0xA3; mem[0x201] = 0x00; // I = 0x300
        mem[0x202] = 0x60; mem[0x203] = 0x1E; // V0 = 30
        mem[0x204] = 0x61; mem[0x205] = 0x0E; // V1 = 14
        mem[0x206] = 0xD0; mem[0x207] = 0x14; // Draw Sprite at (V0, V1)
        mem[0x208] = 0x12; mem[0x209] = 0x08; // Jump to 0x208 (Loop)

        // Pre-fill display with lit pixels for a quick sanity check
        let mut disp = [false; 64 * 32];
        disp[0] = true;    // top-left (0,0)
        disp[65] = true;   // (1,1)

        Chip8 {
            memory: mem,
            v: [0; 16],
            i: 0x300,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            display: disp, // Use the pre-filled test display here
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn get_display_ptr(&self) -> *const bool {
        self.display.as_ptr()
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        let opcode = (hi << 8) | lo;
        self.pc += 2;
        opcode
    }

    pub fn move_v0(&mut self, delta: i8) {
        let current = self.v[0] as i8;
        self.v[0] = (current.wrapping_add(delta)) as u8;
    }

    pub fn move_v1(&mut self, delta: i8) {
        let current = self.v[1] as i8;
        self.v[1] = (current.wrapping_add(delta)) as u8;
    }

    pub fn reset_pc_for_test(&mut self) {
        self.pc = 0x206; 
    }

    fn execute(&mut self, opcode: u16) {
        let op = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nnn = opcode & 0x0FFF;

        match (op, x, y, n) {
            (0, 0, 0xE, 0) => { self.display.fill(false); },
            (0x1, _, _, _) => { self.pc = nnn; },
            (0xA, _, _, _) => { self.i = nnn; },
            (0xD, _, _, _) => {
                // Clear the screen before drawing to avoid trails/ghosting
                self.display.fill(false);

                let x_pos = self.v[x] as usize % 64;
                let y_pos = self.v[y] as usize % 32;
                self.v[0xF] = 0;

                for row in 0..n {
                    let sprite_byte = self.memory[(self.i + row as u16) as usize];
                    for col in 0..8 {
                        let sprite_pixel = (sprite_byte >> (7 - col)) & 1;
                        if sprite_pixel == 1 {
                            let idx = ((y_pos + row as usize) % 32) * 64 + ((x_pos + col as usize) % 64);
                            if self.display[idx] { self.v[0xF] = 1; }
                            self.display[idx] ^= true; 
                        }
                    }
                }
            },
            _ => {} 
        }
    }
}

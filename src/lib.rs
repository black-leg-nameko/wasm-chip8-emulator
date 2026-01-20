use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8 {
    memory: [u8; 4096],    // 4KBのメモリ
    v: [u8; 16],           // V0 - VF 汎用レジスタ
    i: u16,                // インデックスレジスタ
    pub pc: u16,               // プログラムカウンタ
    stack: [u16; 16],      // スタック
    sp: u16,               // スタックポインタ
    display: [bool; 64 * 32], // 64x32ピクセルの画面状態
    delay_timer: u8,
    sound_timer: u8,
}

#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200, // CHIP-8のプログラムは通常0x200から開始
            stack: [0; 16],
            sp: 0,
            display: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.pc == 0 {
            self.execute(0x1200);
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        let opcode = (hi << 8) | lo;
        self.pc += 2;
        opcode
    }

    fn execute(&mut self, opcode: u16) {
        // ここに命令ごとの処理を書いていく（これが一番楽しい作業！）
        let nibble1 = (opcode & 0xF000) >> 12;
        let nnn = opcode & 0x0FFF;

        match nibble1 {
            0x1 => self.pc = nnn, // 1NNN: 指定アドレスへジャンプ
            _ => todo!("Opcode {:04X} はまだ未実装です", opcode),
        }
    }
}

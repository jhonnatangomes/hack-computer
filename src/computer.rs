use wasm_bindgen::prelude::*;

const MEMORY_SIZE: usize = 32 * 1024;
const SCREEN: usize = 16384;
const KEYBOARD: usize = 24576;

#[wasm_bindgen]
pub struct Computer {
    d_reg: i16,
    a_reg: i16,
    pc: u16,
    instruction_memory: [u8; MEMORY_SIZE],
    data_memory: [u8; MEMORY_SIZE],
}

#[wasm_bindgen]
impl Computer {
    pub fn new() -> Computer {
        Computer {
            d_reg: 0,
            a_reg: 0,
            pc: 0,
            instruction_memory: [0; MEMORY_SIZE],
            data_memory: [0; MEMORY_SIZE],
        }
    }
    pub fn keyboard(&self) -> u8 {
        self.data_memory[KEYBOARD]
    }
    pub fn set_keyboard(&mut self, key: u8) {
        self.data_memory[KEYBOARD] = key;
    }
    pub fn screen(&self) -> u8 {
        self.data_memory[SCREEN]
    }
}

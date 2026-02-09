use std::io::{BufReader, Read};
use std::fs::File;

#[derive(Copy, Clone)]
pub struct Register(u8);

pub struct Keyboard {

}

#[derive(Default)]
pub struct Display {
    dimensions: (usize, usize),
    display : Vec<bool>,
}

impl Display {
    pub fn clear(&mut self) {
        let display = &mut self.display;
        for pixel in display {
           *pixel = false; 
        }
    }
}

#[derive(Default)]
pub struct Timer {

}

pub enum Instructions { 
    CLS,
    RET,
    JUMP(usize),
}

pub struct Chip8 {
    mem_map: [u8;4096],
    registers: [Register;16],
    display: Display,
    timer: Timer,
    sound_timer: Timer,
    pc: usize,
    program_start_offset: usize,
}

impl Chip8 {
    pub fn new() -> Chip8{
        Chip8 {
            mem_map: [0_u8;4096],
            registers: [Register(0);16],
            display: Display {
                dimensions: (64, 32),
                display : Vec::from([false;64*32]),
            },
            timer: Timer{

            },
            sound_timer: Timer {},
            pc: 0,
            program_start_offset: 512,
        }
    }

    pub fn show_mem_map(&self, up_to: Option<usize>){
        let mut counter = 0;
        for x in self.mem_map[self.program_start_offset..].iter(){
            if counter % 2 == 0 {
                print!("Instruction {}|| ", counter / 2);
            }
            print!(
            "{:02X?}{}",
            x,
            if counter % 2 == 0 {
                ' '
            }else{
                '\n'
            }
            );
            counter += 1;
            if let Some(x) = up_to {
                if counter - 1 > x * 2 {
                    break;
                }
            }
        }
    }
    fn clear_display(&mut self){
        self.display.clear();    
    }

    fn get_display(&self) -> &Display {
        &self.display
    }

    fn set_display(&mut self, display: Display) {
        self.display = display;
    }

    pub fn load_program() {
         
    }

    pub fn load_program_from_file(&mut self, file_name : &str) {
        let file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(&file);
        reader.read(&mut self.mem_map[self.program_start_offset..]).unwrap();
        
    }

    pub fn execute_program(&mut self) {
        loop{
            let instruction = self.fetch();

        }
    }

    fn fetch(&mut self) -> (u8, u8) {
        self.pc = self.pc + 2;
        (self.mem_map[self.pc - 1], self.mem_map[self.pc - 2])
    }

}



use chip_8::chip8::*;

fn main() {
    let mut chip_8 = Chip8::new();
    chip_8.load_program_from_file("Logo.ch8");
    chip_8.show_mem_map(Some(30));
}

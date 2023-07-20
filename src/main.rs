extern crate sdl2;
use std::time::Duration;
use std::fs::File;
use std::io::Read;

mod display;
mod input;
mod cpu;

fn draw_diagonal_line(width: u32, height: u32) -> Vec<(u32, u32)> {
    let mut coordinates = Vec::new();

    // Calculate the slope of the diagonal line
    let slope = height as f32 / width as f32;

    // Iterate over the x-coordinates
    for x in 0..width {
        // Calculate the corresponding y-coordinate
        let y = (x as f32 * slope) as u32;
        coordinates.push((x, y));
    }

    coordinates
}

fn rom_from_file() -> Vec<u8> {
    let mut rom = Vec::new();
    let mut file = File::open("../roms/ibm-logo.ch8").unwrap();
    file.read_to_end(&mut rom).unwrap();
    rom
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Display::new(&sdl_context);
    let mut input = input::Input::new(&sdl_context);
    let mut cpu = cpu::CPU::new(&display);

    //let rom = rom_from_file();
    //cpu.load_rom(&rom);

    //loop {
    //    if !cpu.emulate_cycle() {
    //        break
    //    }
    //}

    let coordinates = draw_diagonal_line(800, 600);
    while input.poll() {
        for (x, y) in &coordinates {
            display.draw(*x as i32, *y as i32);
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

use crate::modules::cart::Cart;
use crate::modules::cpu::CPU;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use super::bus::Bus;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const SCALE: u16 = 4;

const TILE_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(175, 175, 175),
    Color::RGB(85, 85, 85),
    Color::RGB(0, 0, 0),
];

pub struct Emu {
    paused: bool,
    running: bool,
    pub die: bool,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            paused: false,
            running: true,
            die: false,
        }
    }

    pub fn run(rom_path: String) {
        let mut cpu = CPU::new();
        Cart::load(&mut cpu.bus.cart, &rom_path);

        cpu.init();
        cpu.timer.ticks = 0;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _ttf_context = sdl2::ttf::init().unwrap();

        let main_window = video_subsystem
            .window("Gameboy Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let x_pos = main_window.position().0 + main_window.size().0 as i32;
        let y_pos = main_window.position().1;

        let debug_window = video_subsystem
            .window("Debug", 384, SCREEN_HEIGHT)
            .position(x_pos, y_pos)
            .opengl()
            .build()
            .unwrap();

        let mut canvas1 = main_window.into_canvas().build().unwrap();
        let mut dbg_canvas = debug_window.into_canvas().build().unwrap();

        canvas1.set_draw_color(Color::RGB(0, 0, 0));
        canvas1.clear();
        canvas1.present();

        dbg_canvas.set_draw_color(Color::RGB(17, 17, 17));
        dbg_canvas.clear();
        dbg_canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'gameboyloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::Window {
                        win_event: WindowEvent::Close,
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'gameboyloop,
                    _ => {}
                }
            }

            if !cpu.step() {
                dbg!("CPU STOPED");
                break;
            }

            Self::update_ui(&cpu, &mut dbg_canvas);
        }
        return;
    }

    fn update_debug_window(cpu: &CPU, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let mut x_draw = 0;
        let mut y_draw = 0;
        let mut tile_num = 0;

        // make rectangle
        let rect = sdl2::rect::Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1);
        canvas.set_draw_color(Color::RGB(17, 17, 17));
        canvas.fill_rect(rect).unwrap();

        let address = 0x8000;

        // 384 tiles -> 24 * 16
        for tile_y in 0..24 {
            for tile_x in 0..16 {
                Self::display_tile(
                    cpu,
                    canvas,
                    address,
                    tile_num,
                    x_draw + (tile_x * SCALE),
                    y_draw + (tile_y * SCALE),
                );
                x_draw += 8 * SCALE;
                tile_num += 1;
            }

            x_draw = 0;
            y_draw += 8 * SCALE;
        }
    }

    fn update_ui(cpu: &CPU, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        Self::update_debug_window(cpu, canvas);
        canvas.present();
    }

    fn display_tile(
        cpu: &CPU,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        address: u16,
        tile_num: u16,
        x: u16,
        y: u16,
    ) {
        for tile_y in (0..16).step_by(2) {
            let byte1: u8 = Bus::read(cpu, address + (tile_num * 16) + tile_y);
            let byte2: u8 = Bus::read(cpu, address + (tile_num * 16) + tile_y + 1);

            // println!("Byte1: {:08b} | Byte2: {:08b}", byte1, byte2);

            for bit in (0..7).rev() {
                let hi = !!(byte1 & (1 << bit)) << 1;
                let lo = !!(byte2 & (1 << bit));

                let color = TILE_COLORS[(hi | lo) as usize];

                // draw rectangle
                let rect_x: i32 = (x + (7 - bit) * SCALE) as i32;
                let rect_y = (y + tile_y * SCALE) as i32;
                let rect_w = SCALE as u32;
                let rect_h = SCALE as u32;

                let rect = sdl2::rect::Rect::new(rect_x, rect_y, rect_w, rect_h);
                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
}

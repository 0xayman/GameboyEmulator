use crate::modules::cart::Cart;
use crate::modules::cpu::Cpu;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use super::bus::Bus;
use super::ppu::Ppu;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const SCALE: u16 = 4;

const LINES_PER_FRAME: u32 = 154;
const TICKS_PER_LINE: u32 = 456;
const YRES: i32 = 144;
const XRES: i32 = 160;

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
        let mut cpu = Cpu::new();
        Cart::load(&mut cpu.bus.cart, &rom_path);

        cpu.init();
        Ppu::init(&mut cpu);
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

        let debug_window = video_subsystem
            .window("Debug", 584, 864)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = main_window.into_canvas().build().unwrap();
        let mut dbg_canvas = debug_window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        dbg_canvas.set_draw_color(Color::RGB(17, 17, 17));
        dbg_canvas.clear();
        dbg_canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut prev_frame = 0;

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

            if prev_frame != cpu.bus.ppu.current_frame {
                Self::update_ui(&cpu, &mut dbg_canvas, &mut canvas);
                dbg_canvas.present();
            }

            prev_frame = cpu.bus.ppu.current_frame;
        }
    }

    fn update_debug_window(cpu: &Cpu, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let mut x_draw = 0;
        let mut y_draw = 0;
        let mut tile_num = 0;

        // // make rectangle
        // let rect = sdl2::rect::Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1);
        // canvas.set_draw_color(Color::RGB(17, 17, 17));
        // canvas.fill_rect(rect).unwrap();

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

    fn update_ui(
        cpu: &Cpu,
        debug_canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        let video_buffer = cpu.bus.ppu.video_buffer;

        for line_num in 0..YRES {
            for x in 0..XRES {
                let rect = sdl2::rect::Rect::new(
                    x * SCALE as i32,
                    line_num * SCALE as i32,
                    SCALE as u32,
                    SCALE as u32,
                );

                // Convert video_buffer at index x + (line_num * XRES) to a color
                // it is a u32 value
                // let color = TILE_COLORS[video_buffer[(x + (line_num * XRES)) as usize] as usize];
                let color = video_buffer[(x + (line_num * XRES)) as usize];

                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();
            }
        }
        canvas.present();

        Self::update_debug_window(cpu, debug_canvas);
    }

    fn display_tile(
        cpu: &Cpu,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        address: u16,
        tile_num: u16,
        x: u16,
        y: u16,
    ) {
        for tile_y in (0..16).step_by(2) {
            let byte1: u8 = Bus::read(cpu, address + (tile_num * 16) + tile_y);
            let byte2: u8 = Bus::read(cpu, address + (tile_num * 16) + tile_y + 1);

            for bit in (0..7).rev() {
                let hi = (((byte1 & (1 << bit)) != 0) as u8) << 1;
                let lo = ((byte2 & (1 << bit)) != 0) as u8;

                let color = TILE_COLORS[(hi | lo) as usize];

                // draw rectangle
                let rect_x: i32 = (x + (7 - bit) * SCALE) as i32;
                let rect_y = (y + tile_y / 2 * SCALE) as i32;
                let rect_w = SCALE as u32;
                let rect_h = SCALE as u32;

                let rect = sdl2::rect::Rect::new(rect_x, rect_y, rect_w, rect_h);
                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();
            }
        }
    }

    pub fn delay(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }

    pub fn get_ticks() -> u64 {
        let now = std::time::SystemTime::now();
        now.duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

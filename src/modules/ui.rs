pub mod ui {
    use std::sync::mpsc;

    const SCREEN_WIDTH: u32 = 1024;
    const SCREEN_HEIGHT: u32 = 768;

    pub fn init(tx: mpsc::Sender<String>) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        println!("SDL INIT");

        let _ttf_context = sdl2::ttf::init().unwrap();
        println!("TTF INIT");

        let window = video_subsystem
            .window("Gameboy Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        // loop
        'running: loop {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.present();
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => {
                        tx.send("die".to_string()).unwrap();
                        break 'running;
                    }
                    _ => {}
                }
            }
        }

        println!("\nSDL QUIT");
    }
}

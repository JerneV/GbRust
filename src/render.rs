#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


const GBW : u32 = 160;
const GBH : u32 = 144;

const SCALE : u32 = 1;

const WIDTH: u32 = GBW as u32 * SCALE;
const HEIGHT: u32 = GBH as u32 * SCALE;

struct FBuffer {
    xpos : u32,
    ypos : u32,
}

pub fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Jerne Test")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut fbuf = FBuffer{xpos: 0, ypos: 0};

    pixels.resize_surface(WIDTH, HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            fbuf.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

impl FBuffer {
    // Draw to the Frame Buffer
    fn draw(&mut self, frame: &mut [u8]) {
        //println!("Drawing");
        //Iterate through all pixels of the frame buffer
        for (_i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            self.xpos+=1;
            if self.xpos == WIDTH{
                self.ypos += 1;
                self.xpos = 0;
            }
            if self.ypos == HEIGHT{
                self.ypos = 0;
            }
            let rgba = {
                if self.xpos % 2 == 0{
                    [self.ypos as u8,50,self.ypos as u8,255]
                }else{
                   [0,self.ypos as u8,0,255]
                }
            }; 
            //let rgba = [self.xpos as u8,self.xpos as u8,self.xpos as u8,255]; 
            pixel.copy_from_slice(&rgba);
        }
    }
}
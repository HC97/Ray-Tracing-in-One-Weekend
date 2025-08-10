use std::iter;
use std::rc::Rc;
use std::num::NonZeroU32;

use utils::Interval;
use vector::Vec3;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;

pub mod vector;
pub mod camera;
pub mod sence;
pub mod material;
pub mod utils;
use camera::Camera;
use sence::Sence;

pub struct Renderer {
    width: u32,
    height: u32,
    count: u32,
    samples: u32,
    buffer: Vec<Vec3>,
    camera: Camera,
    world: Sence
}

impl Renderer {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera, world: Sence) -> Self {
        let buffer = Vec::new();
        Self { width, height, count: 1, buffer, camera, world, samples }
    }

    fn render(&mut self, window: &Rc<Window>, width: u32, height: u32) {
        if self.count <= self.samples {
            let count = self.count as f64;
            self.buffer.resize((width * height) as usize, Vec3::default());
            let tex = self.camera.render(&self.world, width, height);
            for (bc, tc) in iter::zip(&mut self.buffer, tex) {
                *bc = *bc * ((count - 1.0) / count) + tc / count;
            }
            println!("Samples: {}", self.count);
            self.count += 1;
            window.request_redraw();
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = Rc::new(
            WindowBuilder::new()
                .with_inner_size(PhysicalSize::new(self.width, self.height))
                .with_resizable(false)
                .build(&event_loop)
                .unwrap()
        );
        
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        surface.resize(
            NonZeroU32::new(self.width).unwrap(),
            NonZeroU32::new(self.height).unwrap(),
        ).unwrap();
    
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);
    
            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested
                } if window_id == window.id() => {
                    let mut buffer = surface.buffer_mut().unwrap();
                    self.render(&window, self.width, self.height);
                    for index in 0..self.buffer.len() {
                        let (r, g, b) = {
                            let color = self.buffer[index];
                            let interval = Interval::new(0.000, 0.999);
                            let (r, g, b) = (
                                interval.clamp(color.x()),
                                interval.clamp(color.y()),
                                interval.clamp(color.z())
                            );
                            let (r, g , b) = (libm::sqrt(r), libm::sqrt(g), libm::sqrt(b));
                            (
                                (r * 255.999) as u32,
                                (g * 255.999) as u32,
                                (b * 255.999) as u32
                            )
                        };
                        buffer[index] = b | (g << 8) | (r << 16);
                    }
                    buffer.present().unwrap();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {}
            }
        }).unwrap();
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

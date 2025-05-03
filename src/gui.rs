use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ActiveEventLoop, ControlFlow},
    window::{Window, WindowId},
    application::ApplicationHandler,
};
use egui_wgpu::wgpu::SurfaceError;
use egui_wgpu::{wgpu, ScreenDescriptor};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
}


pub struct AppState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub scale_factor: f32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(start_window(event_loop).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => (),
        }
    }
}

fn start_window(event_loop: &ActiveEventLoop) -> Option<Window> {
    let window_attributes = Window::default_attributes().with_title("Task App");
    
    let window = Some(event_loop.create_window(window_attributes).unwrap());

    window
}

pub fn start_gui() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();

    event_loop.run_app(&mut app);

}



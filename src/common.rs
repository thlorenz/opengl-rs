extern crate glfw;

use self::glfw::{Action, Context, Key};
use crate::camera::{Camera, CameraMovement};
use std::sync::mpsc::Receiver;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub struct Mouse {
    pub seen: bool,
    pub x: f32,
    pub y: f32,
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse {
            seen: false,
            x: 0.0,
            y: 0.0,
        }
    }
}

pub struct Scene {
    pub ctx: glfw::Glfw,
    pub window: glfw::Window,
    pub camera: Camera,
    mouse: Mouse,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Default for Scene {
    fn default() -> Self {
        let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS.clone()).unwrap();
        ctx.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        ctx.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        ctx.window_hint(glfw::WindowHint::CenterCursor(true));
        #[cfg(target_os = "macos")]
        ctx.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = ctx
            .create_window(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                "Learn OpenGL",
                glfw::WindowMode::Windowed,
            )
            .expect("Create Window");
        window.set_pos(0, 0);
        window.set_focus_on_show(true);
        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mouse = Mouse::default();
        let camera = Camera::default();
        Scene {
            ctx,
            window,
            camera,
            mouse,
            events,
        }
    }
}

impl Scene {
    pub fn move_window_to_left_monitor(&mut self) {
        self.window.set_pos(-(SCREEN_WIDTH as i32), 0);
    }

    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let (x, y) = (x as f32, y as f32);
                    if !self.mouse.seen {
                        self.mouse.x = x;
                        self.mouse.y = y;
                        self.mouse.seen = true;
                    }

                    let dx = x - self.mouse.x;
                    let dy = y - self.mouse.y;
                    self.mouse.x = x;
                    self.mouse.y = y;

                    self.camera.process_mouse_move(dx, dy, true);
                }
                glfw::WindowEvent::Scroll(_, dy) => {
                    self.camera.process_mouse_scroll(dy as f32);
                }
                _ => {}
            }
        }
    }

    pub fn process_input(&mut self, dt: f32) {
        if self.window.get_key(Key::W) == Action::Press {
            self.camera.process_keyboard(CameraMovement::Forward, dt);
        }
        if self.window.get_key(Key::S) == Action::Press {
            self.camera.process_keyboard(CameraMovement::Backward, dt);
        }
        if self.window.get_key(Key::A) == Action::Press {
            self.camera.process_keyboard(CameraMovement::Left, dt);
        }
        if self.window.get_key(Key::D) == Action::Press {
            self.camera.process_keyboard(CameraMovement::Right, dt);
        }
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_should_close(true);
        }
    }
}

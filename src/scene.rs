extern crate glfw;

use self::glfw::{Action, Context, Cursor, Key};
use crate::camera::{Camera, CameraMovement};
use std::sync::mpsc::Receiver;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    seen: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse {
            x: 0.0,
            y: 0.0,
            seen: false,
        }
    }
}

pub struct Scene {
    pub ctx: glfw::Glfw,
    pub window: glfw::Window,
    pub camera: Camera,
    pub dt: f32,
    pub show_depth: bool,
    mouse: Mouse,
    events: Receiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
    ratio: f32,
    last_frame_ts: f64,
    time_to_info: f32,
    delay_toggle_until: f64,
    show_mesh: bool,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Self {
        let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS.clone()).unwrap();
        ctx.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        ctx.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        ctx.window_hint(glfw::WindowHint::CenterCursor(true));
        #[cfg(target_os = "macos")]
        ctx.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = ctx
            .create_window(width, height, "Learn OpenGL", glfw::WindowMode::Windowed)
            .expect("Create Window");
        window.set_pos(0, 0);
        window.set_focus_on_show(true);
        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Normal);
        window.set_cursor(Some(Cursor::standard(glfw::StandardCursor::Crosshair)));

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mouse = Mouse::default();
        let camera = Camera::default();
        let ratio = width as f32 / height as f32;

        let last_frame_ts = ctx.get_time();
        let show_depth = false;

        Scene {
            ctx,
            window,
            camera,
            mouse,
            show_depth,
            events,
            width,
            height,
            ratio,
            last_frame_ts,
            dt: 0.0,
            time_to_info: 0.0,
            delay_toggle_until: 0.0,
            show_mesh: false,
        }
    }
    pub fn move_window_to_left_monitor(&mut self) {
        self.window.set_pos(-(SCREEN_WIDTH as i32), 0);
    }

    pub fn update_camera(&mut self) {
        let time = self.ctx.get_time();
        let dt = (time - self.last_frame_ts) as f32;

        self.process_events();
        self.process_input(time, dt);
        self.show_info(dt);

        self.last_frame_ts = time;
        self.dt = dt;
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let (x, y) = (x as f32, y as f32);

                    if self.window.get_mouse_button(glfw::MouseButtonLeft) == Action::Press {
                        let (dx, dy) = if self.mouse.seen {
                            (x - self.mouse.x, y - self.mouse.y)
                        } else {
                            (0.0, 0.0)
                        };
                        self.camera.process_mouse_move(dx, dy, true);
                        self.mouse.seen = true;
                    }
                    self.mouse.x = x;
                    self.mouse.y = y;
                }
                glfw::WindowEvent::Scroll(_, dy) => {
                    self.camera.process_mouse_scroll(dy as f32);
                }
                _ => {}
            }
        }
    }

    fn process_input(&mut self, time: f64, dt: f32) {
        let process_toggle = self.delay_toggle_until < time;

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
        if process_toggle {
            if self.window.get_key(Key::M) == Action::Press {
                self.show_mesh = !self.show_mesh;
                unsafe {
                    if self.show_mesh {
                        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    } else {
                        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                    }
                }
                self.reset_delay_toggle(time);
            }
            if self.window.get_key(Key::Z) == Action::Press {
                self.show_depth = !self.show_depth;
                self.reset_delay_toggle(time);
            }
        }
    }

    fn reset_delay_toggle(&mut self, time: f64) {
        self.delay_toggle_until = time + 0.25;
    }

    fn show_info(&mut self, dt: f32) {
        self.time_to_info -= dt;
        if self.time_to_info <= 0.0 {
            self.window.set_title(&format!(
                "({:.2}:{:.2}:{:.2}) pitch: {:.2} yaw: {:.2} FPS: {:.0}",
                self.camera.position.x,
                self.camera.position.y,
                self.camera.position.z,
                self.camera.pitch,
                self.camera.yaw,
                (1.0 / dt).round()
            ));
            // show info 5 times a second
            self.time_to_info = 1.0 / 5.0;
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }
}

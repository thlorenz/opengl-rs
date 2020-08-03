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
pub fn init() -> (
    glfw::Glfw,
    glfw::Window,
    Receiver<(f64, glfw::WindowEvent)>,
    Mouse,
    Camera,
) {
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS.clone()).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
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
    // window.set_pos(-(SCREEN_WIDTH as i32), 0);
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
    (ctx, window, events, mouse, camera)
}

pub fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    mouse: &mut Mouse,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::CursorPos(x, y) => {
                let (x, y) = (x as f32, y as f32);
                if !mouse.seen {
                    mouse.x = x;
                    mouse.y = y;
                    mouse.seen = true;
                }

                let dx = x - mouse.x;
                let dy = y - mouse.y;
                mouse.x = x;
                mouse.y = y;

                camera.process_mouse_move(dx, dy, true);
            }
            glfw::WindowEvent::Scroll(_, dy) => {
                camera.process_mouse_scroll(dy as f32);
            }
            _ => {}
        }
    }
}

pub fn process_input(dt: f32, window: &mut glfw::Window, camera: &mut Camera) {
    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(CameraMovement::Forward, dt);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(CameraMovement::Backward, dt);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(CameraMovement::Left, dt);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(CameraMovement::Right, dt);
    }
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}

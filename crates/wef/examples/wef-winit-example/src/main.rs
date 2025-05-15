use std::{cell::RefCell, rc::Rc, time::Duration};

use softbuffer::Surface;
use wef::{
    Browser, BrowserHandler, DirtyRects, ImageBuffer, KeyCode, KeyModifier, MouseButton,
    PaintElementType, Rect, Settings, Size,
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{Ime, Modifiers, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{ModifiersState, NamedKey},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
    window::{Window, WindowId},
};

type BoxError = Box<dyn std::error::Error>;

struct SharedState {
    surface: Surface<Rc<Window>, Rc<Window>>,
    popup: Option<Rect>,
    window: Rc<Window>,
}

struct State {
    scale_factor: f32,
    browser: Browser,
    shared_state: Rc<RefCell<SharedState>>,
}

impl State {
    fn new(window: Window) -> Result<Self, BoxError> {
        let window = Rc::new(window);
        let inner_size = window.inner_size();
        let context = softbuffer::Context::new(window.clone())?;
        let mut surface = Surface::new(&context, window.clone())?;

        surface.resize(
            inner_size.width.try_into().expect("valid surface width"),
            inner_size.height.try_into().expect("valid surface height"),
        )?;

        let scale_factor = window.scale_factor() as f32;
        let shared_state = Rc::new(RefCell::new(SharedState {
            window,
            surface,
            popup: None,
        }));

        // Create the browser instance
        let browser = Browser::builder()
            .size(inner_size.width, inner_size.height)
            .device_scale_factor(scale_factor)
            .url("https://www.rust-lang.org/")
            .handler(MyHandler {
                view_size: Size::default(),
                scale_factor,
                shared_state: shared_state.clone(),
            })
            .build();

        Ok(Self {
            scale_factor,
            browser,
            shared_state,
        })
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
    key_modifiers: KeyModifier,
}

impl App {
    #[inline]
    fn browser_mut(&mut self) -> &mut Browser {
        &mut self.state.as_mut().unwrap().browser
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes().with_inner_size(LogicalSize::new(1024, 768)),
            )
            .unwrap();
        window.set_ime_allowed(true);

        self.state = Some(State::new(window).expect("create window"));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                // Resize the render target
                self.browser_mut().resize(size.width, size.height);
            }
            WindowEvent::CursorMoved { position, .. } => {
                let scale_factor = self.state.as_ref().unwrap().scale_factor;
                let position = position.to_logical::<f32>(scale_factor as f64);
                self.browser_mut()
                    .send_mouse_move_event(position.x as i32, position.y as i32);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let button = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    _ => return,
                };
                let pressed = state.is_pressed();
                self.browser_mut()
                    .send_mouse_click_event(button, !pressed, 1);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (delta_x, delta_y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (50 * x as i32, 50 * y as i32),
                    MouseScrollDelta::PixelDelta(delta) => (delta.x as _, delta.y as _),
                };
                self.browser_mut().send_mouse_wheel_event(delta_x, delta_y);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.key_modifiers = convert_key_modifiers(modifiers);
            }
            WindowEvent::KeyboardInput { event, .. } => match event.logical_key.as_ref() {
                winit::keyboard::Key::Named(named_key) => {
                    if let Some(key_code) = convert_key_code(named_key) {
                        let key_modifiers = self.key_modifiers;
                        self.browser_mut().send_key_event(
                            event.state.is_pressed(),
                            key_code,
                            key_modifiers,
                        );
                    }
                }
                winit::keyboard::Key::Character(s) if event.state.is_pressed() => {
                    for ch in s.chars() {
                        self.browser_mut().send_char_event(ch as u16);
                    }
                }
                _ => {}
            },
            WindowEvent::Ime(ime) => match ime {
                Ime::Preedit(text, range) => {
                    let (start, end) = range.unwrap_or_default();
                    self.browser_mut().ime_set_composition(&text, start, end);
                }
                Ime::Commit(text) => self.browser_mut().ime_commit(&text),
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                if let Ok(buffer) = RefCell::borrow_mut(&*self.state.as_mut().unwrap().shared_state)
                    .surface
                    .buffer_mut()
                {
                    _ = buffer.present();
                }
            }
            _ => (),
        }
    }
}

struct MyHandler {
    scale_factor: f32,
    view_size: Size,
    shared_state: Rc<RefCell<SharedState>>,
}

impl BrowserHandler for MyHandler {
    fn on_paint(
        &mut self,
        type_: PaintElementType,
        _dirty_rects: &DirtyRects,
        image_buffer: ImageBuffer,
    ) {
        let shared_state = &mut RefCell::borrow_mut(&*self.shared_state);
        let source = unsafe {
            std::slice::from_raw_parts(
                image_buffer.as_ptr() as *const u32,
                (image_buffer.width() * image_buffer.height()) as usize,
            )
        };

        match type_ {
            PaintElementType::View => {
                self.view_size = Size {
                    width: image_buffer.width(),
                    height: image_buffer.height(),
                };
                shared_state
                    .surface
                    .resize(
                        image_buffer
                            .width()
                            .try_into()
                            .expect("valid surface width"),
                        image_buffer
                            .height()
                            .try_into()
                            .expect("valid surface height"),
                    )
                    .expect("resize surface");

                let mut buffer = shared_state.surface.buffer_mut().unwrap();
                buffer.copy_from_slice(source);

                buffer.present().unwrap();
            }
            PaintElementType::Popup => {
                let popup = shared_state.popup.unwrap();
                let mut buffer = shared_state.surface.buffer_mut().unwrap();
                let (width, height) = (image_buffer.width(), image_buffer.height());
                let (x, y) = (
                    (popup.x as f32 * self.scale_factor) as usize,
                    (popup.y as f32 * self.scale_factor) as usize,
                );

                for row in 0..height {
                    let source_start = (row * width) as usize;
                    let source_end = source_start + width as usize;
                    let dest_start = ((row + y as u32) * self.view_size.width + x as u32) as usize;
                    let dest_end = dest_start + width as usize;
                    buffer[dest_start..dest_end].copy_from_slice(&source[source_start..source_end]);
                }

                buffer.present().unwrap();
            }
        }
    }

    fn on_title_changed(&mut self, title: &str) {
        self.shared_state.borrow().window.set_title(title);
    }

    fn on_popup_position(&mut self, rect: &Rect) {
        RefCell::borrow_mut(&*self.shared_state).popup = Some(*rect);
    }
}

fn convert_key_code(key: NamedKey) -> Option<KeyCode> {
    match key {
        NamedKey::Backspace => Some(KeyCode::Backspace),
        NamedKey::Delete => Some(KeyCode::Delete),
        NamedKey::Tab => Some(KeyCode::Tab),
        NamedKey::Enter => Some(KeyCode::Enter),
        NamedKey::PageUp => Some(KeyCode::PageUp),
        NamedKey::PageDown => Some(KeyCode::PageDown),
        NamedKey::End => Some(KeyCode::End),
        NamedKey::Home => Some(KeyCode::Home),
        NamedKey::ArrowLeft => Some(KeyCode::ArrowLeft),
        NamedKey::ArrowUp => Some(KeyCode::ArrowUp),
        NamedKey::ArrowRight => Some(KeyCode::ArrowRight),
        NamedKey::ArrowDown => Some(KeyCode::ArrowDown),
        _ => None,
    }
}

fn convert_key_modifiers(modifiers: Modifiers) -> KeyModifier {
    let mut key_modifiers = KeyModifier::empty();
    if modifiers.state().contains(ModifiersState::SHIFT) {
        key_modifiers |= KeyModifier::SHIFT;
    }
    if modifiers.state().contains(ModifiersState::CONTROL) {
        key_modifiers |= KeyModifier::CONTROL;
    }
    if modifiers.state().contains(ModifiersState::ALT) {
        key_modifiers |= KeyModifier::ALT;
    }
    key_modifiers
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop = EventLoop::new()?;

    let mut app = App::default();

    loop {
        let status = event_loop.pump_app_events(Some(Duration::from_millis(10)), &mut app);
        wef::do_message_loop_work();

        if let PumpStatus::Exit(_) = status {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if wef::exec_process()? {
        // Is helper process, exit immediately
        return Ok(());
    }

    // Run the main process
    let settings = Settings::new();
    wef::init(settings)?;
    run()?;
    wef::shutdown();
    Ok(())
}

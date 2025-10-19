use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::x11::EventLoopBuilderExtX11;
use winit::window::{Window as WinitWindow, WindowId};
use winit::event::WindowEvent as WinitWindowEvent;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

// WindowEvent enum exposed to Kotlin
#[derive(uniffi::Enum, Clone)]
pub enum WindowEvent {
    ActivationTokenDone { serial: u64, token: String },
    Resized { width: u32, height: u32 },
    Moved { x: i32, y: i32 },
    CloseRequested,
    Destroyed,
    DroppedFile { path: String },
    HoveredFile { path: String },
    HoveredFileCancelled,
    Focused { focused: bool },
    KeyboardInput,
    ModifiersChanged,
    Ime,
    CursorMoved { x: f64, y: f64 },
    CursorEntered,
    CursorLeft,
    MouseWheel { delta_x: f64, delta_y: f64 },
    MouseInput,
    PinchGesture { delta: f64 },
    PanGesture { delta_x: f32, delta_y: f32 },
    DoubleTapGesture,
    RotationGesture { delta: f32 },
    TouchpadPressure { pressure: f32, stage: i64 },
    AxisMotion { axis: u32, value: f64 },
    Touch,
    ScaleFactorChanged { scale_factor: f64 },
    ThemeChanged { theme: String },
    Occluded { occluded: bool },
    RedrawRequested,
}

// Window wrapper struct that will be exposed to Kotlin
#[derive(uniffi::Object)]
pub struct Window {
    title: String,
    width: u32,
    height: u32,
    event_queue: Arc<Mutex<VecDeque<WindowEvent>>>,
    running: Arc<Mutex<bool>>,
    window_instance: Arc<Mutex<Option<Arc<WinitWindow>>>>,
}

// Application handler for managing the window lifecycle
struct App {
    title: String,
    width: u32,
    height: u32,
    window: Option<Arc<WinitWindow>>,
    event_queue: Arc<Mutex<VecDeque<WindowEvent>>>,
    running: Arc<Mutex<bool>>,
    window_instance: Arc<Mutex<Option<Arc<WinitWindow>>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create the window when the application is resumed
        if self.window.is_none() {
            let window_attributes = WinitWindow::default_attributes()
                .with_title(&self.title)
                .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));
            
            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    let window_arc = Arc::new(window);
                    self.window = Some(Arc::clone(&window_arc));
                    
                    // Store the window in the shared window_instance
                    if let Ok(mut instance) = self.window_instance.lock() {
                        *instance = Some(Arc::clone(&window_arc));
                    }
                }
                Err(e) => {
                    eprintln!("Failed to create window: {}", e);
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WinitWindowEvent,
    ) {
        // Check if we should exit
        if let Ok(running) = self.running.lock() {
            if !*running {
                event_loop.exit();
                return;
            }
        }

        // Convert winit event to our WindowEvent enum and add to queue
        let our_event = match event {
            WinitWindowEvent::ActivationTokenDone { serial, token } => {
                Some(WindowEvent::ActivationTokenDone {
                    serial: format!("{:?}", serial).parse().unwrap_or(0),
                    token: format!("{:?}", token),
                })
            }
            WinitWindowEvent::Resized(size) => {
                Some(WindowEvent::Resized {
                    width: size.width,
                    height: size.height,
                })
            }
            WinitWindowEvent::Moved(position) => {
                Some(WindowEvent::Moved {
                    x: position.x,
                    y: position.y,
                })
            }
            WinitWindowEvent::CloseRequested => {
                event_loop.exit();
                Some(WindowEvent::CloseRequested)
            }
            WinitWindowEvent::Destroyed => Some(WindowEvent::Destroyed),
            WinitWindowEvent::DroppedFile(path) => {
                Some(WindowEvent::DroppedFile {
                    path: path.to_string_lossy().to_string(),
                })
            }
            WinitWindowEvent::HoveredFile(path) => {
                Some(WindowEvent::HoveredFile {
                    path: path.to_string_lossy().to_string(),
                })
            }
            WinitWindowEvent::HoveredFileCancelled => Some(WindowEvent::HoveredFileCancelled),
            WinitWindowEvent::Focused(focused) => Some(WindowEvent::Focused { focused }),
            WinitWindowEvent::KeyboardInput { .. } => Some(WindowEvent::KeyboardInput),
            WinitWindowEvent::ModifiersChanged(_) => Some(WindowEvent::ModifiersChanged),
            WinitWindowEvent::Ime(_) => Some(WindowEvent::Ime),
            WinitWindowEvent::CursorMoved { position, .. } => {
                Some(WindowEvent::CursorMoved {
                    x: position.x,
                    y: position.y,
                })
            }
            WinitWindowEvent::CursorEntered { .. } => Some(WindowEvent::CursorEntered),
            WinitWindowEvent::CursorLeft { .. } => Some(WindowEvent::CursorLeft),
            WinitWindowEvent::MouseWheel { delta, .. } => {
                use winit::event::MouseScrollDelta;
                let (delta_x, delta_y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                };
                Some(WindowEvent::MouseWheel { delta_x, delta_y })
            }
            WinitWindowEvent::MouseInput { .. } => Some(WindowEvent::MouseInput),
            WinitWindowEvent::PinchGesture { delta, .. } => {
                Some(WindowEvent::PinchGesture { delta })
            }
            WinitWindowEvent::PanGesture { delta, .. } => {
                Some(WindowEvent::PanGesture {
                    delta_x: delta.x,
                    delta_y: delta.y,
                })
            }
            WinitWindowEvent::DoubleTapGesture { .. } => Some(WindowEvent::DoubleTapGesture),
            WinitWindowEvent::RotationGesture { delta, .. } => {
                Some(WindowEvent::RotationGesture { delta })
            }
            WinitWindowEvent::TouchpadPressure { pressure, stage, .. } => {
                Some(WindowEvent::TouchpadPressure { pressure, stage })
            }
            WinitWindowEvent::AxisMotion { axis, value, .. } => {
                Some(WindowEvent::AxisMotion { axis, value })
            }
            WinitWindowEvent::Touch(_) => Some(WindowEvent::Touch),
            WinitWindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                Some(WindowEvent::ScaleFactorChanged { scale_factor })
            }
            WinitWindowEvent::ThemeChanged(theme) => {
                Some(WindowEvent::ThemeChanged {
                    theme: format!("{:?}", theme),
                })
            }
            WinitWindowEvent::Occluded(occluded) => Some(WindowEvent::Occluded { occluded }),
            WinitWindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
                Some(WindowEvent::RedrawRequested)
            }
        };

        // Add event to queue
        if let Some(evt) = our_event {
            if let Ok(mut queue) = self.event_queue.lock() {
                queue.push_back(evt);
            }
        }
    }
}

#[uniffi::export]
impl Window {
    /// Creates a new Window with the specified title, width, and height
    #[uniffi::constructor]
    pub fn new(title: String, width: u32, height: u32) -> Arc<Self> {
        Arc::new(Window {
            title,
            width,
            height,
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            running: Arc::new(Mutex::new(false)),
            window_instance: Arc::new(Mutex::new(None)),
        })
    }

    /// Starts the window event loop in a background thread
    pub fn start(&self) {
        let title = self.title.clone();
        let width = self.width;
        let height = self.height;
        let event_queue = Arc::clone(&self.event_queue);
        let running = Arc::clone(&self.running);
        let window_instance = Arc::clone(&self.window_instance);

        // Set running to true
        if let Ok(mut r) = running.lock() {
            *r = true;
        }

        thread::spawn(move || {
            let event_loop = match EventLoop::builder()
                .with_any_thread(true)
                .build() {
                Ok(el) => el,
                Err(e) => {
                    eprintln!("Failed to create event loop: {}", e);
                    return;
                }
            };

            let mut app = App {
                title,
                width,
                height,
                window: None,
                event_queue,
                running,
                window_instance,
            };

            let _ = event_loop.run_app(&mut app);
        });

        // Give the thread a moment to start up
        thread::sleep(std::time::Duration::from_millis(100));
    }

    /// Polls for the next event from the queue (non-blocking)
    /// Return None if no events are available
    pub fn poll_event(&self) -> Option<WindowEvent> {
        if let Ok(mut queue) = self.event_queue.lock() {
            queue.pop_front()
        } else {
            None
        }
    }

    /// Checks if the window is still running
    pub fn is_running(&self) -> bool {
        if let Ok(running) = self.running.lock() {
            *running
        } else {
            false
        }
    }

    /// Stops the window event loop
    pub fn stop(&self) {
        if let Ok(mut running) = self.running.lock() {
            *running = false;
        }
    }

    /// Gets the native window handle as a u64
    /// On X11, this returns the Window XID
    /// On Wayland, this returns the wl_surface pointer as u64
    /// Returns 0 if the window hasn't been created yet
    pub fn get_window_handle(&self) -> u64 {
        if let Ok(instance) = self.window_instance.lock() {
            if let Some(window) = instance.as_ref() {
                // Get the raw window handle
                if let Ok(handle) = window.window_handle() {
                    let raw_handle = handle.as_raw();
                    match raw_handle {
                        #[cfg(target_os = "linux")]
                        RawWindowHandle::Xlib(xlib_handle) => {
                            xlib_handle.window as u64
                        }
                        #[cfg(target_os = "linux")]
                        RawWindowHandle::Wayland(wayland_handle) => {
                            wayland_handle.surface.as_ptr() as u64
                        }
                        _ => {
                            eprintln!("Unsupported window handle type");
                            0
                        }
                    }
                } else {
                    eprintln!("Failed to get window handle");
                    0
                }
            } else {
                // Window not created yet
                0
            }
        } else {
            // Failed to lock
            0
        }
    }
}

// This generates extra Rust code required by UniFFI.
uniffi::setup_scaffolding!();

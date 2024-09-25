extern crate sys_info;

mod endpoint;
mod which;

use propagation::endpoint::{Endpoint, EndpointKind};
use std::default::Default;
use std::option::Option::{None, Some};
use std::string::{ToString};
use winit::event::WindowEvent as WindowEventEnum;
use winit::event::Event::WindowEvent as WindowEventStruct;
use winit::event_loop::{EventLoop, ControlFlow, EventLoopBuilder, EventLoopWindowTarget};
use winit::window::WindowBuilder;
use std::iter;
use std::prelude::v1::derive;
use std::result::Result::{Err, Ok};
use std::time::Instant;
use ::egui::FontDefinitions;
use chrono::Timelike;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use wgpu::{CompositeAlphaMode, InstanceDescriptor};
use winit::event::Event::*;
use egui::{Context, Modifiers, Ui, WidgetText};
use egui_demo_lib::{DemoWindows, is_mobile};
use egui::NumExt;
use egui::text::LayoutJob;
use epi::egui::Widget;
use sys_info::os_type;

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

const PROPAGATION_VERSION_NUMBER: &str = "0.01";

/// A custom event type for the winit app.
enum CustomEvent {
    RequestRedraw,
}

/// This is the repaint signal type that egui needs for requesting a repaint from another thread.
/// It sends the custom RequestRedraw event to the winit event loop.
struct CustomRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<CustomEvent>>);

impl epi::backend::RepaintSignal for CustomRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(CustomEvent::RequestRedraw).ok();
    }
}

fn main() {
    let sys_info = sys_info::os_type().unwrap_or_default();
    println!("{}", sys_info);
    let which_os = which::os();
    println!("which_os = {:?}", which_os);

    let device = Endpoint::new("meme".to_string(), EndpointKind::Recording, 0);
    println!("{:?}", device);
    println!("{}", device);

    let os_infor = os_info::get();
    println!("{}: v{} {}", os_infor.os_type(), os_infor.version(), os_infor.bitness());

    let event_loop: EventLoop<CustomEvent> = EventLoopBuilder::<CustomEvent>::with_user_event().build();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("Propagation")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        });
    let window = builder.build(&event_loop).unwrap();

    let instance_desc: InstanceDescriptor = wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        .. Default::default()
    };

    let instance = wgpu::Instance::new(instance_desc);
    let surface: wgpu::Surface = unsafe { instance.create_surface(&window).unwrap() };

    // WGPU 0.11+ support force fallback (if HW implementation not supported), set it to true or false (optional).
    let adapter: wgpu::Adapter = pollster::block_on(instance.request_adapter(
        &wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            .. Default::default()
        },
        None
    )).unwrap();

    let size = window.inner_size();
    let surface_caps = &surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &surface_config);

    let mut platform: Platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor: window.scale_factor(),
        .. Default::default()
    });

    let mut egui_rpass = RenderPass::new(
        &device,
        surface_format,
        1);

    // let mut demo_app: DemoWindows = egui_demo_lib::DemoWindows::default();

    let mut propagation_app = PropagationWindows::default();

    let start_time = Instant::now();

    // event_loop.set_control_flow(ControlFlow::Wait);
    //
    // event_loop.run(move |event, elwt| match event {
    //     Event::WindowEvent {
    //         event: WindowEvent::CloseRequested,
    //         ..
    //     } => {
    //         println!("The close button was pressed; stopping");
    //         elwt.exit();
    //     }
    //     _ => (),
    // });

    event_loop.run(move |event, elwt: &EventLoopWindowTarget<CustomEvent>, control_flow: &mut ControlFlow| {
        platform.handle_event(&event);

        match event {
            RedrawRequested(..) => {
                platform.update_time(start_time.elapsed().as_secs_f64());

                let output_frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(wgpu::SurfaceError::Outdated) => {
                        // This error occurs when the app is minimized on Windows.
                        // Silently return here to prevent spamming the console with:
                        // "The underlying surface has changed, and therefore the swap chain must be updated"
                        return;
                    },
                    Err(e) => {
                        eprintln!("Dropped frame with error: {}", e);
                        return;
                    }
                };

                let output_view = output_frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Begin to draw the UI frame.
                platform.begin_frame();

                // Draw the demo application.
                // demo_app.ui(&platform.context());
                propagation_app.ui(&platform.context());

                // End the UI frame. We could now handle the output and draw the UI with the backend.
                let full_output = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(full_output.shapes);

                let mut encoder = device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: Some("encoder"),
                    });

                let screen_descriptor = ScreenDescriptor {
                    physical_width: surface_config.width,
                    physical_height: surface_config.height,
                    scale_factor: window.scale_factor() as f32,
                };

                let t_delta: egui::TexturesDelta = full_output.textures_delta;
                egui_rpass
                    .add_textures(&device, &queue, &t_delta)
                    .expect("add texture ok");
                egui_rpass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

                // Record all render passes.
                egui_rpass.execute(
                    &mut encoder,
                    &output_view,
                    &paint_jobs,
                    &screen_descriptor,
                    Some(wgpu::Color::BLACK))
                    .unwrap();

                // Submit the commands.
                queue.submit(iter::once(encoder.finish()));

                // Redraw egui
                output_frame.present();

                egui_rpass
                    .remove_textures(t_delta)
                    .expect("remove texture ok");
            },
            WindowEventStruct { event, .. } => match event {
                WindowEventEnum::Resized(size) => {
                    // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                    // See: https://github.com/rust-windowing/winit/issues/208
                    // This solves an issue where the app would panic when minimizing on Windows.
                    if size.width > 0 && size.height > 0 {
                        surface_config.width = size.width;
                        surface_config.height = size.height;
                        surface.configure(&device, &surface_config);
                    }
                },
                WindowEventEnum::CloseRequested => {
                    // elwt.exit();
                    *control_flow = ControlFlow::Exit;
                },
                _ => {},
            },
            MainEventsCleared | UserEvent(CustomEvent::RequestRedraw) => {
                window.request_redraw();
            },
            _ => {},
        }
    });
}

/// Time of day as seconds since midnight. Used for clock in demo app.
pub fn seconds_since_midnight() -> f64 {
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct PropagationWindows {
    about_is_open: bool,
    about: About,
}

impl Default for PropagationWindows {
    fn default() -> Self {
        Self {
            about_is_open: true,
            about: Default::default(),
        }
    }
}

impl PropagationWindows {
    pub fn ui(&mut self, ctx: &Context) {
        if is_mobile(ctx) {
            self.mobile_ui(ctx);
        } else {
            self.desktop_ui(ctx);
        }
    }

    fn mobile_ui(&mut self, ctx: &Context) {
        if self.about_is_open {
            let screen_size = ctx.input(|i| i.screen_rect.size());
            let default_width = (screen_size.x - 32.0).at_most(400.0);

            let mut close = false;
            egui::Window::new(self.about.name())
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(default_width)
                .default_height(ctx.available_rect().height() - 46.0)
                .vscroll(true)
                .open(&mut self.about_is_open)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    self.about.ui(ui);
                    ui.add_space(12.0);
                    ui.vertical_centered_justified(|ui| {
                        if ui
                            .button(egui::RichText::new("Propagation").size(20.0))
                            .clicked()
                        {
                            close = true;
                        }
                    });
                });
            self.about_is_open &= !close;
        } else {
            self.mobile_top_bar(ctx);
            self.show_windows(ctx);
        }
    }

    fn mobile_top_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let font_size = 16.5;

                // ui.menu_button(egui::RichText::new("⏷ demos").size(font_size), |ui| {
                //     ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
                //     self.demo_list_ui(ui);
                //     if ui.ui_contains_pointer() && ui.input(|i| i.pointer.any_click()) {
                //         ui.close_menu();
                //     }
                // });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    use egui::special_emojis::{GITHUB, TWITTER};
                    ui.hyperlink_to(
                        egui::RichText::new(TWITTER).size(font_size),
                        "https://twitter.com/massivelivefun",
                    );
                    ui.hyperlink_to(
                        egui::RichText::new(GITHUB).size(font_size),
                        "https://github.com/dezzyne/propagation",
                    );
                });
            });
        });
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file_menu_button(ui);
            });
        });

        self.show_windows(ctx);
    }

    fn show_windows(&mut self, ctx: &Context) {
        self.about.show(ctx, &mut self.about_is_open);
    }

    fn file_menu_button(&mut self, ui: &mut Ui) {
        let organize_shortcut =
            egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
        let reset_shortcut =
            egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

        // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
        // or else they would only be checked if the "File" menu was actually open!

        if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
        }

        if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
        }

        ui.menu_button("File", |ui: &mut Ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);

            // On the web the browser controls the zoom
            #[cfg(not(target_arch = "wasm32"))]
            {
                egui::gui_zoom::zoom_menu_buttons(ui, None);
                ui.separator();
            }

            if ui
                .add(
                    egui::Button::new("Organize Windows")
                        .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
                )
                .clicked()
            {
                ui.ctx().memory_mut(|mem| mem.reset_areas());
                ui.close_menu();
            }

            if ui
                .add(
                    egui::Button::new("Reset egui memory")
                        .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
                )
                .on_hover_text("Forget scroll, positions, sizes etc")
                .clicked()
            {
                ui.ctx().memory_mut(|mem| *mem = Default::default());
                ui.close_menu();
            }
        });

        ui.menu_button("About", |ui: &mut Ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);

            if ui
                .add(
                    egui::Button::new("About Propagation")
                )
                .on_hover_text("Open a window with information about Propagation")
                .clicked()
            {
                self.about_is_open = true;
                ui.close_menu();
            }
        });
    }
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct About {}

impl Draw for About {
    fn name(&self) -> &'static str {
        ""
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .collapsible(false)
            .resizable(true)
            .open(open)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

impl View for About {
    fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Propagation");
        ui.label(format!("Propagation"));

        use egui::special_emojis::{GITHUB, TWITTER};
        ui.hyperlink_to(
            format!("{GITHUB} MASSIVELIVEFUN on GitHub"),
            "https://github.com/massivelivefun/",
        );
        ui.hyperlink_to(
            format!("{TWITTER} @massivelivefun"),
            "https://twitter.com/massivelivefun/",
        );

        // ui.separator();

        let os_info = os_info::get();

        ui.label(format!{
            "os name: {} {} {}",
             os_info.os_type(),
             os_info.version(),
             os_info.bitness()
        });
    }
}

/// Something to view
pub trait Draw {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &Context, open: &mut bool);
}

pub trait View {
    fn ui(&mut self, ui: &mut Ui);
}

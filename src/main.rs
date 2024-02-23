// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::text::{FontDescriptor, FontFamily};
use druid::widget::{Flex, TextBox};
use druid::{
    AppDelegate, AppLauncher, commands, Command, Color, Data, DelegateCtx, Env, Lens,
    LocalizedString, Menu, Widget, WidgetExt, WindowDesc, WindowId, Target, Handled
};

mod config;
mod tab;

use crate::tab::{TabConfig, DynamicTabData};
use crate::config::Config;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Zero Note");


#[derive(Clone, Data, Lens)]
struct AppState {
    config: Config,
    tab_config: TabConfig,
    advanced: DynamicTabData,
    multi: Arc<String>
}

struct Delegate {
    windows: Vec<WindowId>,
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env)
    -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            println!("{:?}", file_info.path());
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    data.multi = Arc::new(s);
                }
                Err(e) => {
                    println!("Error opening file: {e}");
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .menu(make_menu)
        .window_size((500.0, 600.0));

    // create the initial app state
    let initial_state = AppState {
        config: Config::new(),
        tab_config: Default::default(),
        advanced: DynamicTabData::new(2),
        multi: "".to_string().into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .delegate(Delegate {
            windows: Vec::new(),
        })
        .launch(initial_state)
        .expect("Failed to launch");
}

fn build_root_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_flex_child(
            TextBox::multiline()
                .with_placeholder("text some content")
                .with_font(FontDescriptor::new(FontFamily::SERIF))
                .with_text_size(16.0)
                .lens(AppState::multi)
                .expand_width()
                .expand_height(),
            1.0,
        )
        .padding(6.0)
}


#[allow(unused_assignments, unused_mut)]
fn make_menu<T: Data>(_window: Option<WindowId>, _data: &AppState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.entry(druid::platform_menus::mac::application::default())
    }
    #[cfg(any(
        target_os = "windows",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "macos"
    ))]
    {
        base = base.entry(druid::platform_menus::win::file::default());
    }
    base.entry(
        Menu::new(LocalizedString::new("common-menu-edit-menu"))
            .entry(druid::platform_menus::common::undo())
            .entry(druid::platform_menus::common::redo())
            .separator()
            .entry(druid::platform_menus::common::cut())
            .entry(druid::platform_menus::common::copy())
            .entry(druid::platform_menus::common::paste()),
    )
}

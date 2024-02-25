// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::text::{FontDescriptor, FontFamily};
use druid::widget::{Flex, TextBox};
use druid::{AppLauncher, Color, LocalizedString, Widget, WidgetExt, WindowDesc};

mod config;
mod delegate;
mod menu;
mod state;
mod tab;

use crate::config::Config;
use crate::delegate::Delegate;
use crate::menu::make_menu;
use crate::state::AppState;
use crate::tab::DynamicTabData;


const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Zero Note");


fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .menu(make_menu)
        .window_size((800.0, 600.0));

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
        .delegate(Delegate::new())
        .launch(initial_state)
        .expect("Failed to launch");
}

fn build_root_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_flex_child(
            TextBox::multiline()
                .with_placeholder("Start from here")
                .with_font(FontDescriptor::new(FontFamily::MONOSPACE))
                .with_text_size(16.0)
                .lens(AppState::multi)
                .expand_width()
                .expand_height(),
            1.0,
        )
        .padding(0.0)
}




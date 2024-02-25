use druid::{
    AppDelegate, commands, Command, DelegateCtx, Env, WindowHandle, WindowId, Target, Handled
};

use crate::state::AppState;


pub struct Delegate {
    windows: Vec<WindowId>,
}

impl Delegate {
    pub fn new() -> Self {
        Delegate {
            windows: Vec::new(),
        }
    }
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
            data.open_file(file_info.path());
            return Handled::Yes;
        }
        Handled::No
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _handle: WindowHandle,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        self.windows.push(id);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        if let Some(pos) = self.windows.iter().position(|x| *x == id) {
            self.windows.remove(pos);
        }
    }
}
use editor::{app_with_editor, EditorOpenSetting};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value_t = false)]
    pub fullscreen: bool,
}

fn main() {
    let args = Arguments::parse();
    let open_settings = match args.fullscreen {
        true => EditorOpenSetting::FullScreen,
        false => EditorOpenSetting::Windowed,
    };

    app_with_editor(open_settings).run();
}

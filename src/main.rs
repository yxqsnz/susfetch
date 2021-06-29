use std::env;
mod colors;
use colors::Colors;
use colors::Colors::*;

use susfetch::{
    IOResult,
    get_item_from_c_buf,
};

#[derive(Debug)]
pub struct SusFetch {
    kernel: String,
    shell: String,
    wm: String,
    host: String,
    os: String,
    terminal: String
}
impl SusFetch {
    fn default() -> IOResult<Self> {
        let kernel = susfetch::get_kernel_version()?;

        let host = format!(
            "{}@{}",
            get_item_from_c_buf!(getlogin_r),
            get_item_from_c_buf!(gethostname)
        );

        let os = susfetch::get_distro();
 
        let shell = match env::var("SHELL") {
            Ok(var) => {
                let split: Vec<_> = var.split('/').collect();
                match split.last() {
                    Some(sh) => sh.to_string(),
                    _ => "unknown".to_string()
                }
            }
            _ => "unknown".to_string()
        };

        let wm = env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or(
                env::var("DESKTOP_SESSION").unwrap_or("unknown".to_owned())
            );

        let terminal = env::var("TERM").unwrap_or("xterm".to_owned());

        Ok(Self {
            kernel,
            host,
            os,
            wm,
            shell,
            terminal
        })
    }

    fn format(&mut self) {
        if self.wm == *"unknown" {
            self.wm = Colors::colorize(Red, "unknown");
        }

        if self.shell == *"unknown" {
            self.shell = Colors::colorize(Red, "unknown");
        }
    }
    fn show(&self) {
        println!(
            r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣤⣤⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀{}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⠟⠉⠉⠉⠉⠉⠉⠉⠙⠻⢶⣄⠀⠀   -------⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣷⡀⠀⠀⠀{}: {}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡟⠀⣠⣶⠛⠛⠛⠛⠛⠛⠳⣦⡀⠀⠘⣿⡄⠀⠀{}: {}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⠁⠀⢹⣿⣦⣀⣀⣀⣀⣀⣠⣼⡇⠀⠀⠸⣷⠀⠀{}: {}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⡏⠀⠀⠀⠉⠛⠿⠿⠿⠿⠛⠋⠁⠀⠀⠀⠀⣿⡄ {}: {}
⠀⠀    ⠀⠀⢠⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡇⠀{}: {}
      ⠀⠀⣸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣧⠀
⠀⠀⠀⠀⠀⠀⠀⢸⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⠀
⠀⠀⠀⠀⠀⠀⠀⣾⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀
⠀⠀⠀⠀⠀⠀⠀⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀
⠀⠀⠀⠀⠀⠀⢰⣿⠀⠀⠀⠀⣠⡶⠶⠿⠿⠿⠿⢷⣦⠀⠀⠀⠀⠀⠀⠀⣿⠀
⠀⠀⣀⣀⣀⠀⣸⡇⠀⠀⠀⠀⣿⡀⠀⠀⠀⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⣿⠀
⣠⡿⠛⠛⠛⠛⠻⠀⠀⠀⠀⠀⢸⣇⠀⠀⠀⠀⠀⠀⣿⠇⠀⠀⠀⠀⠀⠀⣿⠀
⢻⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⡟⠀⠀⢀⣤⣤⣴⣿⠀⠀⠀⠀⠀⠀⠀⣿⠀
⠈⠙⢷⣶⣦⣤⣤⣤⣴⣶⣾⠿⠛⠁⢀⣶⡟⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡟⠀
          ⠀⠀⠀⠀⠈⣿⣆⡀⠀⠀⠀⠀⠀⠀⢀⣠⣴⡾⠃⠀
⠀        ⠀⠀⠀⠀⠀⠀⠈⠛⠻⢿⣿⣾⣿⡿⠿⠟⠋⠁⠀⠀"#,
            self.host,
            Colors::colorize(Green, "OS"),
            self.os,
            Colors::colorize(Green, "Kernel"),
            self.kernel,
            Colors::colorize(Green, "Shell"),
            self.shell,
            Colors::colorize(Green, "WM"),
            self.wm,
            Colors::colorize(Green, "Terminal"),
            self.terminal
        )
    }
    pub fn run(&mut self) {
        self.format();
        self.show();
    }
}
fn main() -> IOResult<()> {
    let mut sus = SusFetch::default()?;
    sus.run();
    Ok(())
}

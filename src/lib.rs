use std::process::Command;
mod get_item_from_c_buf;

pub type IOResult<T> = Result<T, std::io::Error>;

pub fn get_kernel_version() -> IOResult<String> {
    // read /proc/version to get the the kernel version
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(r#"cat /proc/version|grep -oE "Linux version ([A-z]|[0-9]|\.|-)*""#)
        .output()?
        .stdout;

    Ok(String::from_utf8(output).unwrap()) // basically no chance of failing here
}

pub fn get_distro() -> String {
    // parse /etc/os-release to get the kernel version
    match std::fs::read_to_string("/etc/os-release") {
        Ok(string) => {
            string.split('\n')
                .filter(|string| string.contains("PRETTY_NAME"))
                .collect::<String>()
                .replace("PRETTY_NAME=", "")
                .replace("\"", "")
        }

        _ => "Linux".to_owned()
    }
}


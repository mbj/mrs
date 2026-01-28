/// Print the Sway IPC socket path
#[derive(Debug, clap::Parser)]
pub struct App {}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let socket = crate::sink::SwaySocket::find()?;
        println!("{socket}");
        Ok(())
    }
}

/// Stop SRT receiver for a single output.
#[derive(Debug, clap::Parser)]
pub struct App {
    name: crate::OutputName,
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let unit = crate::sink::SystemdUnit::new(&self.name);

        let output = unit.stop().output()?;
        if !output.success() {
            return Err(crate::Error::SystemdStopFailed {
                unit: unit.to_string(),
                status: output.status,
            });
        }

        println!("Stopped {}", unit);

        Ok(())
    }
}

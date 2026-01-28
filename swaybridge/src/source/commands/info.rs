/// Show source status and configured sinks
#[derive(Debug, clap::Parser)]
pub struct App {}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let config = crate::source::config::Source::load_default()?;

        let mut table = comfy_table::Table::new();

        table
            .load_preset(comfy_table::presets::NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec!["name", "host", "user"]);

        for (name, sink) in &config.sinks {
            table.add_row(vec![
                name.to_string(),
                sink.host.to_string(),
                sink.user.to_string(),
            ]);
        }

        println!("{table}");

        Ok(())
    }
}

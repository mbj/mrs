use std::collections::BTreeMap;

/// Show sink status and outputs
#[derive(Debug, clap::Parser)]
pub struct App {
    /// Output JSON for machine parsing
    #[clap(long)]
    json: bool,
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let socket = crate::sink::SwaySocket::find()?;
        let sink_config = crate::sink::config::Sink::load_default()?;
        let sway_outputs = socket.get_outputs()?;

        if self.json {
            return self.run_json(&sink_config, &sway_outputs);
        }

        let mut rows: BTreeMap<String, [String; 3]> = BTreeMap::new();

        for output in &sway_outputs {
            let port = sink_config
                .output
                .get(&output.name)
                .map(|config| config.port.to_string())
                .unwrap_or_default();

            let (mode, transform) = match &output.current_mode {
                Some(mode) => (mode.to_string(), output.transform.to_string()),
                None => ("(no mode)".to_string(), String::new()),
            };

            rows.insert(
                output.name.to_string(),
                [mode, transform, port],
            );
        }

        for (name, config) in &sink_config.output {
            let name = name.to_string();

            if rows.contains_key(&name) {
                continue;
            }

            rows.insert(name, [
                "?".to_string(),
                "?".to_string(),
                config.port.to_string(),
            ]);
        }

        let mut table = comfy_table::Table::new();

        table
            .load_preset(comfy_table::presets::NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec!["name", "mode", "transform", "port"]);

        for (name, [mode, transform, port]) in &rows {
            table.add_row(vec![name, mode, transform, port]);
        }

        println!("{table}");

        Ok(())
    }

    fn run_json(
        &self,
        sink_config: &crate::sink::config::Sink,
        sway_outputs: &[crate::sink::Output],
    ) -> Result<(), crate::Error> {
        let outputs: Vec<JsonOutput> = sink_config
            .output
            .iter()
            .filter_map(|(name, config)| {
                let sway_output = sway_outputs.iter().find(|o| &o.name == name)?;
                let mode = sway_output.current_mode?;

                Some(JsonOutput {
                    name: name.to_string(),
                    width: mode.width,
                    height: mode.height,
                    port: config.port.get(),
                })
            })
            .collect();

        let info = JsonInfo { outputs };

        println!("{}", serde_json::to_string(&info).expect("JSON serialization"));

        Ok(())
    }
}

#[derive(serde::Serialize)]
struct JsonInfo {
    outputs: Vec<JsonOutput>,
}

#[derive(serde::Serialize)]
struct JsonOutput {
    name: String,
    width: u32,
    height: u32,
    port: u16,
}

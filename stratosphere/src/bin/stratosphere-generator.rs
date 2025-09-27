use stratosphere::resource_specification::*;
use stratosphere::token::*;

#[derive(clap::Parser)]
struct App {
    service_identifiers: Vec<String>,
}

fn main() {
    let app = <App as clap::Parser>::parse();

    let mut service_identifiers = std::collections::BTreeSet::new();

    for string in app.service_identifiers.iter() {
        if !service_identifiers.insert(std::convert::TryFrom::try_from(string.as_str()).unwrap()) {
            panic!("Service identifer {string} is duplicate!")
        }
    }

    let stream =
        stratosphere::token::token_stream(Target::for_services(instance(), service_identifiers));

    match syn::parse2(stream.clone()) {
        Ok(parsed) => println!("{}", prettyplease::unparse(&parsed)),
        Err(error) => {
            eprintln!("error while parsing generated code: {error}, printing raw stream");
            println!("{stream}");
            panic!()
        }
    }
}

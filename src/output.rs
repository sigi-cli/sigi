/// Output formats supported by Sigi.
#[derive(Clone, Copy, PartialEq)]
pub enum OutputFormat {
    /// Comma-separated values.
    Csv,
    /// Human readable formats. Accepts a "noise level" for how much to output.
    Human(NoiseLevel),
    /// JSON (JavaScript Object Notation).
    Json,
    /// Print nothing at all.
    Silent,
    /// Tab-separated values.
    Tsv,
}

/// How much noise (verbosity) should be used when printing to standard output.
#[derive(Clone, Copy, PartialEq)]
pub enum NoiseLevel {
    Verbose,
    Normal,
    Quiet,
}

impl OutputFormat {
    pub fn log(&self, labels: Vec<&str>, values: Vec<Vec<&str>>) {
        let joining = |sep: &str| {
            let sep = sep.to_string();
            move |tokens: Vec<&str>| tokens.join(&sep)
        };
        match &self {
            OutputFormat::Csv => {
                let csv = joining(",");
                println!("{}", csv(labels));
                values
                    .into_iter()
                    .for_each(|line| println!("{}", csv(line)))
            }
            OutputFormat::Human(noise) => match noise {
                NoiseLevel::Verbose => {
                    // Print all values separated by a single space.
                    values
                        .into_iter()
                        .for_each(|line| println!("{}", line.join(" ")));
                }
                NoiseLevel::Normal => {
                    // Print only first two values e.g. (num, item) separated by a single space.
                    values.into_iter().for_each(|line| {
                        if let (Some(label), Some(item)) = (line.get(0), line.get(1)) {
                            println!("{}: {}", label, item);
                        } else if let Some(info) = line.get(0) {
                            println!("{}", info);
                        }
                    });
                }
                NoiseLevel::Quiet => values.into_iter().for_each(|line| {
                    // Print only second value (item) separated by a single space.
                    if let Some(message) = line.get(1) {
                        println!("{}", message);
                    }
                }),
            },
            OutputFormat::Json => {
                println!("json: TODO")
            }
            OutputFormat::Silent => (),
            OutputFormat::Tsv => {
                let tsv = joining("\t");
                println!("{}", tsv(labels));
                values
                    .into_iter()
                    .for_each(|line| println!("{}", tsv(line)))
            }
        }
    }
}

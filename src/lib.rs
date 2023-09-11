mod alphabet;
mod log_macros;

use clap::{Parser, ValueEnum};
use core::fmt::Arguments;
use easy_error::{self, ResultExt};
use nanoid;
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

pub trait NanoidCliLog {
    fn output(self: &Self, args: Arguments);
    fn warning(self: &Self, args: Arguments);
    fn error(self: &Self, args: Arguments);
}

pub struct NanoidCliTool<'a> {
    log: &'a dyn NanoidCliLog,
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Disable colors in output
    #[arg(long = "no-color", short = 'n', env = "NO_CLI_COLOR")]
    no_color: bool,

    #[arg(long, short = 'l')]
    length: Option<usize>,

    #[arg(long, short = 'c')]
    count: Option<usize>,

    /// The output file
    #[arg(value_name = "OUTPUT_FILE")]
    output_file: Option<PathBuf>,

    /// Alphabet
    #[arg(long, short = 'a', num_args = 0..=1, default_value_t = Alphabet::Safe, default_missing_value = "safe", value_enum)]
    alphabet: Alphabet,
}

#[derive(ValueEnum, Clone)]
enum Alphabet {
    Dec,
    Hex,
    Alpha,
    AlphaFull,
    Safe,
}

impl Cli {
    fn get_output(&self) -> Result<Box<dyn Write>, Box<dyn Error>> {
        match self.output_file {
            Some(ref path) => File::create(path)
                .context(format!(
                    "Unable to create file '{}'",
                    path.to_string_lossy()
                ))
                .map(|f| Box::new(f) as Box<dyn Write>)
                .map_err(|e| Box::new(e) as Box<dyn Error>),
            None => Ok(Box::new(io::stdout())),
        }
    }
}

impl<'a> NanoidCliTool<'a> {
    pub fn new(log: &'a dyn NanoidCliLog) -> NanoidCliTool {
        NanoidCliTool { log }
    }

    pub fn run(
        self: &mut Self,
        args: impl IntoIterator<Item = std::ffi::OsString>,
    ) -> Result<(), Box<dyn Error>> {
        let cli = match Cli::try_parse_from(args) {
            Ok(m) => m,
            Err(err) => {
                output!(self.log, "{}", err.to_string());
                return Ok(());
            }
        };

        let alphabet: &[char] = match cli.alphabet {
            Alphabet::Dec => &alphabet::DECIMAL,
            Alphabet::Hex => &alphabet::HEXADECIMAL,
            Alphabet::Alpha => &alphabet::ALPHA_NUMERIC,
            Alphabet::AlphaFull => &alphabet::ALPHA_NUMERIC_FULL,
            Alphabet::Safe => &alphabet::URL_SAFE,
        };

        for _ in 0..cli.count.unwrap_or(1) {
            let id = nanoid::format(nanoid::rngs::default, alphabet, cli.length.unwrap_or(21));

            writeln!(cli.get_output()?, "{}", id)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        struct TestLogger;

        impl TestLogger {
            fn new() -> TestLogger {
                TestLogger {}
            }
        }

        impl NanoidCliLog for TestLogger {
            fn output(self: &Self, _args: Arguments) {}
            fn warning(self: &Self, _args: Arguments) {}
            fn error(self: &Self, _args: Arguments) {}
        }

        let logger = TestLogger::new();
        let mut tool = NanoidCliTool::new(&logger);
        let args: Vec<std::ffi::OsString> = vec!["".into(), "--help".into()];

        tool.run(args).unwrap();
    }
}

#[derive(Debug, clap::Parser)]
#[command(about = "A solver for Advent of Code 2024", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Fetch the input file for a given day on a given year
    #[command(arg_required_else_help = true)]
    Fetch { year: u16, day: u8 },

    /// Solve any given day(s) of Advent of Code 2024
    Solve {
        /// Specify the year
        year: u16,

        /// Specify the day (omit day to solve all)
        day: Option<u8>,

        /// Choose a custom input file
        #[arg(short, long, value_name = "FILE", default_value = "input.txt")]
        input: String,
    },
}

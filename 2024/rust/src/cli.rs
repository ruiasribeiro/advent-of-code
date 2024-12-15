#[derive(Debug, clap::Parser)]
#[command(about = "A solver for Advent of Code 2024", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Fetch the input file for a given day
    #[command(arg_required_else_help = true)]
    Fetch { day: u8 },

    /// Solve any given day(s) of Advent of Code 2024
    Solve {
        /// Solve a specific day (omit day to solve all)
        day: Option<u8>,

        /// Solve a custom input
        #[arg(short, long, value_name = "FILE", default_value = "input.txt")]
        input: String,
    },
}

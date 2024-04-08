// command line modules
use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::{arg, Parser};

/// Inspect and convert UKAEA CuV posvol binaries
///
/// Very simple reader for posvol binaries, skipping the need to open a
/// special viewer or sort it out manually just to check simple properties.
///
/// Allows for 1:1 conversion to ASCII, but also JSON and a more readable
/// text file.
///
/// Examples
/// --------
///
///  Print a summary of dimension properties
///     $ posvol plot_fmesh_104.bin
///
///  Convert to ASCII
///     $ posvol plot_fmesh_104.bin --ascii
///     $ posvol plot_fmesh_104.bin --ascii --pretty
///
///  Convert to JSON
///     $ posvol plot_fmesh_104.bin --json
///
///  Change file names to "myfile"
///     $ posvol plot_fmesh_104.bin --json --ascii --output myfile
///
/// Notes
/// -----
///
/// The endian is assumed to be the same as the native type of the system
/// this tool is run on. If needed, an option can be provided in future
/// updates.
#[derive(Parser)]
#[command(
    verbatim_doc_comment,
    arg_required_else_help(true),
    after_help("Note: --help shows more information and examples"),
    term_width(76),
    hide_possible_values(true),
    override_usage("posvol <file> [options]"),
    styles=custom_style()
)]
pub struct Cli {
    // * Positional
    /// Path to posvol binary file
    #[arg(name = "file")]
    pub file: String,

    /// Name of output files
    ///
    /// Defaults to `posvol.<ext>`, and will automatically set the relevant
    /// extension.
    #[arg(help_heading("Conversion options"))]
    #[arg(short, long)]
    #[arg(value_name = "name")]
    #[arg(default_value = "posvol")]
    pub output: String,

    /// Generate a JSON file
    #[arg(help_heading("Conversion options"))]
    #[arg(short, long)]
    pub json: bool,

    /// Generate an ASCII file
    #[arg(help_heading("Conversion options"))]
    #[arg(short, long)]
    pub ascii: bool,

    /// Format the ASCII file for readability
    #[arg(help_heading("Conversion options"))]
    #[arg(short, long)]
    pub pretty: bool,

    // * Flags
    /// Verbose logging (-v, -vv)
    ///
    /// If specified, the default log level of INFO is increased to DEBUG (-v)
    /// or TRACE (-vv). Errors and Warnings are always logged unless in quiet
    /// (-q) mode.
    #[arg(short, long)]
    #[arg(action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Supress all log output (overrules --verbose)
    #[arg(short, long)]
    pub quiet: bool,
}

/// Customise the colour styles for clap v4
fn custom_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Cyan.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Magenta.on_default())
}

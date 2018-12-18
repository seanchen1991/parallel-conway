//! Command-line interface and command-line argument parsing.
//! Utilizes the clap crate under the hood.

const WIDTH_OPT: &str = "WIDTH";
const HEIGHT_OPT: &str = "HEIGHT";
const SPEED_OPT: &str = "SPEED";

/// All options that can be specified by the user using the CLI
pub struct Options {
    /// Specifies how many cells wide the Board is
    pub width: usize,
    /// Specifies how many cells tall the Board is
    pub height: usize,
    /// Speed factor of the animation
    pub speed: f64,
}

/// Parses command-line arguments into one of the possible Options.
/// If help/version is specified or an error occurs, it will be displayed
/// to the user and the process will exit.
pub fn parse_options() -> Options {
    use clap::*;

    let parser = app_from_crate!()
        .setting(AppSettings::NextLineHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name(WIDTH_OPT)
                .short("l")
                .long("width")
                .help("Sets how many cells wide the board is")
                .default_value("200"),
        )
        .arg(
            Arg::with_name(HEIGHT_OPT)
                .short("h")
                .long("height")
                .help("Sets how many cells tall the board is")
                .default_value("200"),
        )
        .arg(
            Arg::with_name(SPEED_OPT)
                .short("s")
                .long("speed")
                .help("Sets animation speed")
                .default_value("1.0"),
        );

    let matches = parser.get_matches();

    Options {
        width: value_t_or_exit!(matches, WIDTH_OPT, usize),
        height: value_t_or_exit!(matches, HEIGHT_OPT, usize),
        speed: value_t_or_exit!(matches, SPEED_OPT, f64),
    }
}
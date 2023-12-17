use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(help_template = "\
{before-help}{name} {version}
{author}
{about}
{usage-heading} {usage}

{all-args}{after-help}
")]
#[allow(clippy::module_name_repetitions)]
#[non_exhaustive]
pub struct AppArgs {
    /// File to edit
    pub file: Option<String>,
}

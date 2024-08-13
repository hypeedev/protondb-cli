use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
    pub(crate) query: Vec<String>,
    #[clap(short, long, default_value_t = 5, help = "Number of results to display")]
    pub(crate) count: u8,
    #[clap(short, long, default_value_t = 5, help = "Number of reports to display")]
    pub(crate) reports: u8,
    #[clap(short = 'I', long, help = "Show game images (using \"viuer\" crate, full resolution images are displayed only in some terminals)")]
    pub(crate) images: bool
}

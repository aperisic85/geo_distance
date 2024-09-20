use clap::Parser;
pub fn parse_args() -> Args {
    let args = Args::parse();
    args
}
#[derive(Debug, Parser)]
#[command(version,about,long_about=None)]
pub struct Args {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long)]
    pub radius: i32,
}

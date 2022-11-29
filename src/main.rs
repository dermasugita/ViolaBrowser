mod vcf2bedpe;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert VCF files into BEDPE files.
    Vcf2bedpe(Vcf2bedpe),
}

#[derive(Args)]
pub struct Vcf2bedpe {
    vcf: Vec<std::path::PathBuf>
}

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Vcf2bedpe(vcf2bedpe) => {
            vcf2bedpe::main(vcf2bedpe);
        }
    }
}

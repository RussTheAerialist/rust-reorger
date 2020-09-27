use glob::Pattern;
use reorger::FileMover;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, short = "n")]
    dry_run: bool,

    #[structopt(long, short, default_value = "*")]
    glob: String,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about = "Split the current directory's files into subdirectories")]
    Split,

    #[structopt(about = "Move all files from subdirectories into this directory")]
    Unsplit,

    #[structopt(about = "Samples files from a directory, copying them into a subdirectory")]
    Sample {
        #[structopt(long, short, default_value=".")]
        source: String,
        nth: u32
    },
}

fn get_file_mover(dry_run: bool) -> Box<dyn FileMover> {
    if dry_run {
        Box::new(reorger::DryRunFileMover {})
    } else {
        Box::new(reorger::OsFileMover {})
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();
    let mover = get_file_mover(opts.dry_run);
    let glob_processor = Pattern::new(&opts.glob)?;

    match opts.command {
        Command::Split => reorger::split(mover.as_ref(), &glob_processor)?,
        Command::Unsplit => reorger::unsplit(mover.as_ref(), &glob_processor)?,
        Command::Sample { source, nth } => reorger::sample(mover.as_ref(), &glob_processor, &source, nth)?,
    };

    Ok(())
}

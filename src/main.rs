use structopt::StructOpt;
use reorg::FileMover;
use glob::Pattern;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, short="n")]
    dry_run: bool,

    #[structopt(long, short, default_value="*")]
    glob: String,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about="Split the current directory's files into subdirectories")]
    Split,

    #[structopt(about="Move all files from subdirectories into this directory")]
    Unsplit,
}

fn get_file_mover(dry_run: bool) -> Box<dyn FileMover> {
    if dry_run {
        Box::new(reorg::DryRunFileMover { })
    } else {
        Box::new(reorg::OsFileMover { })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();
    let mover = get_file_mover(opts.dry_run);
    let glob_processor = Pattern::new(&opts.glob)?;

    match opts.command {
        Command::Split => reorg::split(&mover, &glob_processor)?,
        Command::Unsplit => reorg::unsplit(&mover, &glob_processor)?,
    };

    Ok(())
}

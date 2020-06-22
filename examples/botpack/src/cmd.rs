use gumdrop::Options;
use std::path::PathBuf;

pub(crate) fn parse() -> Command {
    let cmd = Command::parse_args_default_or_exit();
    if cmd.help_requested() {
        println!("{}", Command::usage());
        std::process::exit(0);
    }

    if !cmd.template && !cmd.print && cmd.write.is_none() {
        println!("{}", Command::usage());
        std::process::exit(0);
    }

    cmd
}

#[derive(Debug, Options)]
pub struct Command {
    #[options(help = "print help message")]
    pub help: bool,

    #[options(help = "display configuration template")]
    pub template: bool,

    #[options(help = "print current configuration")]
    pub print: bool,

    #[options(meta = "file", help = "write configuration to binary")]
    pub write: Option<PathBuf>,
}

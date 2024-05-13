pub use clap;

use clap::{ArgMatches, Args, Command, FromArgMatches, Parser};

use crate::error::AnyResult;

#[derive(Debug, Args)]
pub struct StudArgs {
    /// log filter. overrides verbose/quiet levels
    #[arg(short = 'V')]
    pub filter: Option<String>,

    /// log verbosity level, repeatable
    #[arg(short, action = clap::ArgAction::Count, default_value_t = 0)]
    pub verbose: u8,

    /// log quiet level, repeatable
    #[arg(short, action = clap::ArgAction::Count, default_value_t = 0)]
    pub quiet: u8,
}

fn parse<A: FromArgMatches>(cmd: &mut Command, matches: &mut ArgMatches) -> A {
    match A::from_arg_matches_mut(matches) {
        Ok(a) => a,
        Err(e) => e.format(cmd).exit(),
    }
}

async fn init_inner(module: &'static str, stud_args: StudArgs) -> AnyResult<()> {
    crate::logger::init(
        module,
        stud_args.verbose as i8 - stud_args.quiet as i8,
        None,
    )?;
    Ok(())
}

pub async fn init(module: &'static str) -> AnyResult<()> {
    let cmd = Command::new(module);
    let mut cmd = StudArgs::augment_args(cmd);
    let mut matches = cmd.get_matches_mut();

    let stud_args = parse::<StudArgs>(&mut cmd, &mut matches);

    init_inner(module, stud_args).await
}

pub async fn init_with_args<A: Parser>(module: &'static str) -> AnyResult<A> {
    let cmd = A::command();
    let mut cmd = StudArgs::augment_args(cmd);
    let mut matches = cmd.get_matches_mut();

    let stud_args = parse::<StudArgs>(&mut cmd, &mut matches);
    let args = parse::<A>(&mut cmd, &mut matches);

    init_inner(module, stud_args).await?;

    Ok(args)
}

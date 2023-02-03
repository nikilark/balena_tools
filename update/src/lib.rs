use clap::*;
use common::*;

#[derive(Parser, Debug)]
#[command(author, version, about="Updates cached list of devices", long_about = None, trailing_var_arg=true)]
struct Args {
    // To update with specific fleet
    #[arg(
        long = "fleet",
        help = "Update cache with specific fleet before operation"
    )]
    fleet: Option<String>,
}

pub struct UpdateCommand {}
impl BalenaCommand for UpdateCommand {
    fn execute(&self, args: Vec<String>) {
        let args = Args::parse_from(args);
        update_cache(args.fleet);
    }
}

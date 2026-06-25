use std::env;
use std::path::Path;

mod cli;
mod dt_tools;
mod version;

fn main() {

    let mut user_args = cli::UserArgs::new();

    user_args.parse(env::args_os());
    user_args.checks();

    let in_file_path = Path::new(&user_args.ifile);
    let dt = dt_tools::DeviceTree::find(&in_file_path);
    dt.extract(&in_file_path, user_args.ofile);

}

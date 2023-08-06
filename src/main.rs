use clap::Parser;
use dev::{App, func::Run};
fn main() {
    let args = App::parse();
    args.run();
}

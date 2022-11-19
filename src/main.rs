use clap::Parser;
use registry::{Hive, Security};
use std::process::Command;

#[derive(Parser)]
struct Args {
    id: String,
}

fn banner() {
    let banner = "
     █████╗ ███╗   ██╗██╗   ██╗██████╗ ███████╗███████╗██╗  ██╗
    ██╔══██╗████╗  ██║╚██╗ ██╔╝██╔══██╗██╔════╝██╔════╝██║ ██╔╝
    ███████║██╔██╗ ██║ ╚████╔╝ ██║  ██║█████╗  ███████╗█████╔╝ 
    ██╔══██║██║╚██╗██║  ╚██╔╝  ██║  ██║██╔══╝  ╚════██║██╔═██╗ 
    ██║  ██║██║ ╚████║   ██║   ██████╔╝███████╗███████║██║  ██╗
    ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝
    ";
    println!("{}", banner);
}

fn check_anydesk_scheme() -> bool {
    Hive::ClassesRoot
        .open("AnyDesk\\DefaultIcon", Security::Read)
        .expect("error: anydesk wasn't found");

    return true;
}

fn main() {
    banner();
    let args = Args::parse();
    check_anydesk_scheme();

    println!("Connecting to {}...", args.id);

    let _command = Command::new("explorer.exe")
        .arg("anydesk:".to_owned() + &args.id)
        .spawn()
        .expect("failed to open anydesk");
}

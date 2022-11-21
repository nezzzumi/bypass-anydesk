use clap::Parser;
use registry::{Hive, Security};
use std::process::Command;

#[derive(Parser)]
#[command(name = "Anydesk Bypass")]
#[command(author = "Nezumi <constshi0n@proton.me>")]
#[command(version = "1.0")]
#[command(about = "Bypasses AnyDesk popup", long_about = None)]
struct Args {
    /// AnyDesk target id
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

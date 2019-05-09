pub mod config;
pub mod content;
pub mod opts;
pub mod server;
pub mod util;

use structopt::StructOpt;
use config::Config;
use opts::Opt;
use server::Server;

fn main() {
    let opts = Opt::from_args();
    let addr = util::get_address().unwrap();

    let cfg = Config{iface: addr, port: 3000};
    let s = Server::new(&cfg);

    let gen_addr = &s.generated_address;

    let c = content::get(&opts).unwrap();

    println!("Scan the following qr code on your phone to start your download");
    println!("Make sure your phone is connected to thesame wi-fi");
    println!("{}", gen_addr);
    qr2term::print_qr(gen_addr).unwrap();

    s.serve(c);
}

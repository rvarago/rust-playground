use reader::Reader;
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let conf = Config {
        host: Ipv4Addr::new(127, 0, 0, 1).into(),
        port: 8080,
        debug: true,
    };

    program().run_with(&conf)
}

#[derive(Debug)]
struct Config {
    host: IpAddr,
    port: u16,
    debug: bool,
}

type Address = String;

#[derive(Debug)]
struct Server {
    address: Address,
}

#[derive(Debug)]
struct ServerHandle;

type Cfg<A> = Reader<Config, A>;

fn program() -> Cfg<()> {
    dump_conf()
        .then(start_logger)
        .then(make_address)
        .and_then(start_server)
        .map(run_server)
        .void()
}

fn dump_conf() -> Cfg<()> {
    Cfg::new(|conf| println!("> dump_conf: conf={:?}", conf))
}

fn start_logger() -> Cfg<()> {
    Cfg::new(|_| println!("> start_logger"))
}

fn make_address() -> Cfg<String> {
    Cfg::new(|conf| {
        println!("> make_address");
        format!("{}:{}", conf.host, conf.port)
    })
}

fn start_server(address: String) -> Cfg<Server> {
    Cfg::new(|_conf| {
        println!("> start_server: address={}", address);
        Server { address }
    })
}

fn run_server(server: Server) -> ServerHandle {
    println!("> run_server: server={:?}", server);
    ServerHandle
}

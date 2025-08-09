use std::io::{self, Write};
use std::{
    env,
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{Sender, channel},
    thread,
}; // from str allows you to convert from str to any type

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

const MAX: u16 = 65535;
impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }
        let flag = args[1].clone();
        if let Ok(ip_addr) = IpAddr::from_str(&flag) {
            return Ok(Arguments {
                flag: "".to_string(),
                ipaddr: ip_addr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    " Usage -j to select the number of threads you want 
                \r\n -h or -help to show you this message
                "
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid IpAddress type, must be either v4 or v6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Arguments {
                    flag,
                    ipaddr,
                    threads,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap()
            }
            Err(_) => {}
        };

        if MAX - port <= num_threads {
            break;
        }
        port += num_threads;
    }
}

pub fn ip_sniffer() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let argements = Arguments::new(&args).unwrap_or_else(|err_msg| {
        if err_msg.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments {}", program, err_msg);
            process::exit(0);
        }
    });

    let num_threads = argements.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || scan(tx, i, argements.ipaddr, num_threads));
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v)
    }
}

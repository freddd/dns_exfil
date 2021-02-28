use std::net::UdpSocket;

use clap::{App, Arg};
use dns_parser::Packet;

mod discord;
use log::{error, info};
use rayon::prelude::*;
mod slack;

trait Output: Sync + Send {
    fn execute(&self, message: String);
}

struct Console {}

impl Output for Console {
    fn execute(&self, message: String) {
        info!("{}", message)
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("dns-exfil")
        .version("1.0")
        .author("freddd")
        .about("small service used for dns-exfiltration")
        .arg(
            Arg::with_name("bind-addr")
                .short("ba")
                .long("bind-addr")
                .takes_value(true)
                .default_value("0.0.0.0:53")
                .help("Address to bind to")
                .env("BIND_ADDR"),
        )
        .arg(
            Arg::with_name("discord")
                .short("d")
                .long("discord")
                .takes_value(true)
                .help("Discord hook to send lookup request to")
                .env("DISCORD_HOOK"),
        )
        .arg(
            Arg::with_name("slack")
                .short("s")
                .long("slack")
                .takes_value(true)
                .help("Slack hook to send lookup request to")
                .env("SLACK_HOOK"),
        )
        .get_matches();

    let bind_addr = matches.value_of("bind-addr").unwrap();
    let socket = UdpSocket::bind(bind_addr)?;

    let mut outputs: Vec<Box<dyn Output>> = vec![Box::new(Console {})];

    if let Some(hook) = matches.value_of("slack") {
        outputs.push(Box::new(slack::Slack::new(hook.to_string())));
    }

    if let Some(hook) = matches.value_of("discord") {
        outputs.push(Box::new(discord::Discord::new(hook.to_string())));
    }

    loop {
        let mut buf = [0; 2048];
        let (_amt, _src) = socket.recv_from(&mut buf)?;
        match Packet::parse(&buf) {
            Ok(p) => p.questions.into_par_iter().for_each(|q| {
                let sub = q
                    .qname
                    .to_string()
                    .split('.')
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_string();

                outputs.par_iter().for_each(|o| o.execute(sub.clone()))
            }),
            Err(e) => {
                error!("error parsing packet: {:#?}", e)
            }
        }
    }
}

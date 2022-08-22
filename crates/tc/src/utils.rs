use crate::Result;
use std::collections::HashMap;
use std::process::{Command, Output};

#[macro_export]
macro_rules! run {
// run macro
    ($($arg:tt)*) => {{
        let out = $crate::run_output!($($arg)*);
        out.map(|_|())
    }}
}

#[macro_export]
macro_rules! run_output {
    ($($arg:tt)*) => {
        $crate::utils::run(format!($($arg)*))
    }
}

pub fn run(v: String) -> Result<Output> {
    // log all cmds
    //dbg!(&v);

    let cmd = v.clone();
    let mut cmd = cmd.split_whitespace();
    let output = Command::new(cmd.next().expect("Tried to run an empty command"))
        .args(cmd.collect::<Vec<&str>>())
        .output()?;
    if !output.stderr.is_empty() {
        eprintln!(
            "Error while running cmd: {:?}\nerr: {}",
            v,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(output)
}

#[test]
fn tifconfig() {
    dbg!(ifconfig().unwrap());
}
// ifconfig
pub fn ifconfig() -> Result<Vec<Interface>> {
    let raw_data = std::fs::read_to_string("/proc/net/dev")?;

    //TODO: actually parse statue
    raw_data
        .lines()
        .skip(2)
        .filter_map(|l| l.split(':').next())
        .map(|name| {
            Ok(Interface {
                name: name.trim().to_string(),
                status: Status::Down,
            })
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug)]
pub struct Interface {
    pub name: String,
    status: Status,
}

impl Interface {
    pub fn is_up(&self) -> bool {
        self.status == Status::Up
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Up,
    Down,
}

// ss
#[test]
fn tss() {
    dbg!(ss().unwrap());
}

pub fn ss() -> Result<HashMap<String, Vec<Connection>>> {
    let raw_net_table = run_output!("ss -n -t -p  state established")
        .map(|out| String::from_utf8(out.stdout))??;

    let mut net_table = HashMap::new();

    let mut parse = |row: &str| -> Option<()> {
        let mut row = row.split_whitespace();
        let laddr_lport = row.nth(2)?;
        let raddr_rport = row.next()?;
        let process = row.next()?;

        let mut laddr_lport = laddr_lport.split(':');
        let laddr = laddr_lport.next()?;
        let lport = laddr_lport.next()?;

        let mut raddr_rport = raddr_rport.split(':');
        let raddr = raddr_rport.next()?;
        let rport = raddr_rport.next()?;

        let process = process.split('\"').nth(1)?.split('\"').next()?;
        let net_entry: &mut Vec<Connection> = net_table
            .entry(process.to_string())
            .or_insert_with(Vec::new);
        net_entry.push(Connection::new(laddr, lport, raddr, rport));

        Some(())
    };

    for row in raw_net_table.lines().skip(1) {
        let _ = parse(row);
    }

    Ok(net_table)
}

#[derive(Debug)]
pub struct Connection {
    pub laddr: String,
    pub lport: String,
    pub raddr: String,
    pub rport: String,
}

impl Connection {
    fn new(laddr: &str, lport: &str, raddr: &str, rport: &str) -> Connection {
        Connection {
            laddr: laddr.to_string(),
            lport: lport.to_string(),
            raddr: raddr.to_string(),
            rport: rport.to_string(),
        }
    }
}

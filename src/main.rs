use std::io::{BufRead, BufReader, Write};
use std::time::Duration;
use subprocess::*;

const ERR: u128 = 107492042;

fn main() {
    let mut p = Popen::create(
        &["python3", "/Users/alejandro/CLionProjects/fast/main.py"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::Pipe,
            ..Default::default()
        },
    )
        .expect("couldn't spawn child command");

    let mut lines = BufReader::new(p.stdout.as_ref().unwrap()).lines();
    std::thread::sleep(Duration::from_millis(500)); // give bluetoothctl some time to start until issue the scan on command

    loop {
        let mut ii = String::new();
        let _ = std::io::stdin().read_line(&mut ii).unwrap();
        let i: u128 = ii.trim().replace(" ", "").replace("\n", "").parse().unwrap_or(ERR);

        if i == ERR {
            println!("Error: Ingresa un número válido");
            continue;
        }

        let _ = p.stdin.as_ref().unwrap().write_all(format!("{i}\n").as_bytes());
        println!("{}", lines.next().unwrap().unwrap());
        if i == 20 { break; }
    }

    if let Some(exit_status) = p.poll() {
        // the process has finished
    } else {
        // it is still running, terminate it
        p.terminate().unwrap();
    }
}
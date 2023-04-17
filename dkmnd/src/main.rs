extern crate daemonize;

use std::fs::File;

use daemonize::Daemonize;

fn main() {
    let stdout = File::create("/tmp/dkmnd.out").unwrap();
    let stderr = File::create("/tmp/dkmnd.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/dkmnd.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => daemon(),
        Err(e) => eprintln!("Error, {}", e),
    }
}

fn daemon() {
    println!("successfully initalized daemon")
}

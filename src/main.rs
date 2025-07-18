extern crate daemonize;

use std::fs::File;

use daemonize::Daemonize;

fn main() {
    let stdout = File::create("./test/vineyard.out").unwrap();
    let stderr = File::create("./test/vineyard.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("./test/vineyard.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("./test/") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Started the Vineyard daemon!"),
        Err(e) => panic!("oh shit it failed!!: {}",e),
    }
}
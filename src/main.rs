extern crate dotenv;
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate schedule;
extern crate tri;
extern crate void;

use std::process::exit;
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;

use error_chain::ChainedError;
use schedule::{Agenda, Job};
use tri::{Result, ResultExt, Tri};
use void::{unreachable, Void};

fn main() {
    match run() {
        Ok(void) => unreachable(void),
        Err(err) => {
            error!("{}", err.display_chain());
            exit(1)
        }
    }
}

fn run() -> Result<Void> {
    dotenv::dotenv().ok();
    pretty_env_logger::init().chain_err(
        || "Couldn't start logger",
    )?;

    let tri = Arc::new(Tri::new_env()?);

    // Add all the users to the DB.
    tri.add_missing_users()?;

    // Start the scheduler loop on another thread.
    let sched_tri = tri.clone();
    spawn(move || run_scheduler(sched_tri));

    // Listen for messages.
    tri.listen()
}

fn run_scheduler(tri: Arc<Tri>) {
    let mut a = Agenda::new();

    // Every hour, check for new users.
    a.add(Job::new(
        || {
            let r = tri.add_missing_users();
            if let Err(err) = r {
                error!("{}", err.display_chain());
            }
        },
        "0 0 * * * *".parse().unwrap(),
    ));

    // Every Monday at 12:35, message users about their tasks.
    // 12:35 was chosen by the highly judicious KSSWSH heuristic.
    // TODO: This is UTC, we need localtime.
    a.add(Job::new(
        || {
            info!("asdfasdf");
            let r = tri.remind_all_users();
            if let Err(err) = r {
                error!("{}", err.display_chain());
            }
        },
        "0 35 17 * * 1".parse().unwrap(),
    ));

    // Loop forever!
    loop {
        a.run_pending();
        sleep(Duration::from_millis(500));
    }
}

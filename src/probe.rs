use futures::Stream;
use std::sync::mpsc::Sender;
use std::time::Duration;

use tokio_core::reactor::{Core, Interval};
use curl::easy::Easy;

use state::State;

pub fn start(tx: Sender<State>, probe_url: &str, millis: u64) {

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let interval_stream = Interval::new(Duration::from_millis(millis), &handle).unwrap();

    let mut prev_state = State::Off;

    let stream = interval_stream.for_each(|_| {
        let new_state = probe(probe_url);

        if new_state != prev_state {
            println!("state: {:?}", new_state);
            tx.send(new_state).unwrap();;
            prev_state = new_state;
        }

        Ok(())
    });

    core.run(stream).unwrap();
}


fn probe(probe_url: &str) -> State {
    let mut easy = Easy::new();
    easy.url(probe_url).unwrap();
    match easy.perform() {
        Ok(_) => State::On,
        Err(_) => State::Off,
    }
}

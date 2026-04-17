#[derive(Debug)]
struct ConnectionState {
    connected: bool,
    retry_count: u8,
}

impl ConnectionState {
    fn on_failure(&mut self) {
        self.connected = false;
        self.retry_count += 1;
    }

    fn on_success(&mut self) {
        self.connected = true;
        self.retry_count = 0;
    }
}

fn main() {
    let mut state = ConnectionState {
        connected: false,
        retry_count: 0,
    };

    state.on_failure();
    state.on_success();
    println!("{state:?}");
}

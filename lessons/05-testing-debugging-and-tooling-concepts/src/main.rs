#[derive(Debug, PartialEq)]
enum WifiState {
    Disconnected,
    Connecting,
    Connected,
}

fn next_state(current: WifiState, event: &str) -> WifiState {
    match (current, event) {
        (WifiState::Disconnected, "begin") => WifiState::Connecting,
        (WifiState::Connecting, "success") => WifiState::Connected,
        (WifiState::Connecting, "fail") => WifiState::Disconnected,
        (state, _) => state,
    }
}

fn main() {
    println!("{:?}", next_state(WifiState::Disconnected, "begin"));
}

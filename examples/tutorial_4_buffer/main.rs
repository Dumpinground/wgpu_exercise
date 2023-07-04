use pollster::block_on;

// mod challenge;
mod standard;

fn main() {
    if cfg!(feature = "challenge") {
        // pollster::block_on(challenge::run());
    } else {
        pollster::block_on(standard::run());
    }
}
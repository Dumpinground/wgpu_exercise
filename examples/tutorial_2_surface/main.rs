mod standard;

fn main() {
    pollster::block_on(standard::run());
}

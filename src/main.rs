mod engine;

use engine::Engine;

fn main() {
    let program = [
        (0x1 << 24) | (0x02 << 16) | (0x03 << 8), // Instruction #1
        (0x1 << 24) | (0x07 << 16) | (0x08 << 8), // Instruction #2
    ];
    let engine = Engine::new(&program);
    engine.run();
}

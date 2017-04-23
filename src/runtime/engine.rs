use super::operations::{Operation, Execute};

pub struct Engine<'a> {
    instructions: &'a [u32],
}

impl<'a> Engine<'a> {
    pub fn new(instructions: &[u32]) -> Engine {
        Engine {
            instructions: instructions
        }
    }

    pub fn run(&self) {
        println!("Starting program!");
        for instr in self.instructions.iter() {
            let op = Operation::parse(*instr);
            println!("Operation: {:?}\nResult: {}", op, op.execute());
        }
    }
}
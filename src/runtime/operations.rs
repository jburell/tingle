#[derive(Debug)]
pub enum Operation {
    Add(Add)
}

pub trait Execute {
    // TODO: impl state mgmt and different way of dealing with results
    fn execute(&self) -> u32;
}

const ADD_OPCODE:u8 = 0x01;

#[derive(Debug)]
pub struct Add {
    op_1: u8,
    op_2: u8,
}

impl Add {
    fn new(op_1: u8, op_2: u8) -> Add {
        Add {
            op_1: op_1,
            op_2: op_2,
        }
    }
}

impl Execute for Add {
    fn execute(&self) -> u32 {
        (self.op_1 + self.op_2) as u32
    }
}

impl Operation {
    pub fn parse(instruction: u32) -> Operation {
        match Operation::parse_msb(instruction) {
            ADD_OPCODE => {
                let op1 = Operation::parse_msb_plus_one(instruction);
                let op2 = Operation::parse_msb_plus_two(instruction);
                Operation::Add(Add::new(op1, op2))
            }
            _ => unimplemented!()
        }
    }

    #[inline]
    fn parse_msb(instruction: u32) -> u8 {
        (instruction >> 24 & 0xff) as u8
    }

    #[inline]
    fn parse_msb_plus_one(instruction: u32) -> u8 {
        (instruction >> 16 & 0xff) as u8
    }

    #[inline]
    fn parse_msb_plus_two(instruction: u32) -> u8 {
        (instruction >> 8 & 0xff) as u8
    }

    /*#[inline]
    fn parse_lsb(instruction: u32) -> u8 {
        (instruction & 0xff) as u8
    }*/
}

impl Execute for Operation {
    fn execute(&self) -> u32 {
        match *self {
            Operation::Add(ref op) => op.execute()
        }
    }
}
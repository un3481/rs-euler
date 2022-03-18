
use crate::bytecode::{
    ByteCode
};

struct ThreadIter {
    alive: bool,
    location: usize,
    bytecode: &ByteCode,
    scope: HashMap<&str, isize>,
    stack: Vec<isize>
}

mod ThreadIter {

    pub fn new(
        code: &ByteCode,
        index: usize
    ) -> ThreadIter {
        ThreadIter {
            alive: true,
            location: index,
            scope: HashMap::new(),
            stack: vec![]
        }
    }
}

impl ThreadIter {

    fn goto(self: &Self, index: usize) -> &Self {
        *self.location = index;
        self
    }

    fn next(self: &Self) -> Some(Instruction) {
        *self.location = *self.location + 1;
        self.bytecode[self.location]
    }
}

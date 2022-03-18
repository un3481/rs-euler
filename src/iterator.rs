
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

impl ThreadIter {

    pub fn new(
        code: &ByteCode,
        index: usize
    ) -> Self {
        ThreadIter {
            alive: true,
            location: index,
            scope: HashMap::new(),
            stack: vec![]
        }
    }

    fn goto(&self, index: usize) -> Self {
        self.location = index;
        self
    }

    fn next(&self) -> Option<Instruction> {
        self.location = self.location + 1;
        self.bytecode[self.location]
    }
}


use crate::bytecode::{
    ByteCode
};

struct ThreadIter {
    alive: bool,
    index: usize,
    bytecode: &ByteCode,
    scope: HashMap<&str, isize>,
    stack: Vec<isize>
}

impl ThreadIter {

    pub fn new(
        bytecode: &ByteCode,
        index: usize
    ) -> Self {
        ThreadIter {
            alive: true,
            index: index,
            bytecode: bytecode,
            scope: HashMap::new(),
            stack: vec![]
        }
    }

    fn goto(&self, index: usize) -> Self {
        self.index = index;
        self
    }

    fn next(&self) -> Option<Instruction> {
        self.index = self.index + 1;
        self.bytecode[self.index]
    }
}


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

    fn read(&self) -> Option<Instruction> {
        let line = self.bytecode.get(self.index);
        if let Some(_)=line {} else {
            self.active = false;
        };
        line
    }

    pub fn goto(&self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn next(&self) -> Self {
        self.index = self.index + 1;
        self
    }
}

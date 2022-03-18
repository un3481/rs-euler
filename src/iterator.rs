
use crate::bytecode::{
    ByteCode
};

struct ThreadIter {
    alive: bool,
    index: usize,
    bytecode: &ByteCode,
    scope: HashMap<&str, isize>,
    stack: Vec<isize>,
    block: Vec<usize>
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
            stack: vec![],
            block: vec![]
        }
    }

    pub fn goto(&mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn next(&mut self) -> Self {
        self.index = self.index + 1;
        self
    }

    fn read(&self) -> Option<Instruction> {
        let line = self.bytecode.get(self.index);
        if let Some(_)=line {} else {
            self.active = false;
        };
        line
    }

    pub fn eval(&mut self) -> Option<isize> {
        if let Some(instruct)=self.read() {
            match ByteCode::eval(self, instruct) {
                Ok(value) => {},
                Err(error) => {self.alive = false},
            }
        }
    }
}

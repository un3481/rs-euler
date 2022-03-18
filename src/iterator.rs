
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

    pub fn goto(&self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn next(&self) -> Self {
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

    pub fn eval() -> Option<isize> {
        if let Some(instruct)=self.read() {
            match Code::eval(
                &mut self,
                instruct.0,
                instruct.1
            ) {
                Ok(value) => value,
                Err(error) => {self.alive = false},
            }
        }
    }
}

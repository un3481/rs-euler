
use crate::bytecode::{
    ByteCode
};

struct ThreadIter {
    alive: bool,
    index: usize,
    bytecode: &ByteCode,
    scope: HashMap<usize, isize>,
    stack: Vec<isize>,
    block: Vec<usize>,
    error: Option<(usize, &str)>
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
            block: vec![],
            error: None
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
        if let Some(op)=self.read() {
            match self.execute(op) {
                Err(error) => {
                    self.error = Some(error)
                    self.alive = false
                },
                _ => (),
            }
        }
    }

    fn execute(
        &mut self,
        op: Instruction
    ) -> Result<isize, Error> {
        let (command, args) = op;
        match command {
            0 => Eval::push(self, []),
            1 => Eval::set(self, []),
        }
    }
}

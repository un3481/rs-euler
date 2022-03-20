
use crate::bytecode::{
    ByteCode,
    Instruction
};

type ParentIter = Box<(usize, ScopeIter)>;

struct ScopeIter {
    parent: Option<ParentIter>,
    names: [isize; 256],
    block: [(u8, usize); 128],
    stack: [isize; 32],
    flags: [u8; 32],
    blen: u8,
    slen: u8,
    flen: u8
}

struct ThreadIter {
    alive: bool,
    index: usize,
    bytecode: ByteCode,
    scope: ScopeIter,
    error: Option<(usize, &str)>
}

impl ThreadIter {

    #[inline(always)]
    pub fn new(
        bytecode: ByteCode,
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

    #[inline(always)]
    pub fn goto(&mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    #[inline(always)] 
    pub fn next(&mut self) -> Self {
        self.goto(self.index + 1);
        self
    }

    #[inline(always)]
    fn read(&self) -> Option<(u8, Vec<isize>)> {
        let line = self.bytecode.get(self.index);
        if let Some(_)=line {} else {
            self.active = false;
        };
        line
    }

    #[inline(always)]
    pub fn eval(&mut self) -> () {
        if let Some(instruction)=self.read() {
            if self.alive {
                match self.execute(op) {
                    (false, error) => {
                        self.error = Some(error);
                        self.alive = false;
                    },
                    _ => (),
                }
            }
        }
    }

    #[inline(always)]
    fn execute(
        &mut self,
        instruction: (u8, isize)
    ) -> Result<isize, Error> {
        let (command, args) = op;
        match command {
            0 => Stack::push(self, args),
            1 => Stack::pop(self),
            2 => Scope::set(self, args),
            3 => Scope::get(self, args),
            31 => Math::add(self),
            32 => Math::sub(self),
            33 => Math::mul(self),
            34 => Math::div(self),
            41 => If::start(self),
            42 => If::end(self),
            43 => If::else(self),
            51 => Loop::start(self),
            52 => Loop::end(self),
            53 => Loop::break(self),
            54 => Loop::continue(self),
            91 => Fun::declare(self, args),
            92 => Fun::end(self),
            93 => Fun::return(self),
            94 => Fun::call(self, args),
            _ => Error::exit(self, "invalid instruction"),
        }
    }
}

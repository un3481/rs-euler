
use crate::bytecode::{
    ByteCode,
    Instruction
};

type ParentScope = Box<(usize, ThreadScope)>;

struct ThreadScope {
    parent: Option<ParentScope>,
    names: [isize; 256],
    block: [u8; 128],
    stack: [isize; 16],
    flags: [u8; 8],
    blen: u8,
    slen: u8
};

type ThreadError = (usize, &str);

struct GreenThread {
    alive: bool,
    index: usize,
    bytecode: &ByteCode,
    scope: ThreadScope,
    error: Option<ThreadError>
};

impl GreenThread {

    #[inline(always)]
    pub fn new(
        bytecode: &ByteCode,
        index: usize
    ) -> Self {
        GreenThread {
            alive: true,
            index: index,
            bytecode: bytecode,
            scope: ThreadScope::new(),
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
        self.goto(self.index + 1)
    }

    #[inline(always)]
    fn read(&self) -> Option<Instruction> {
        let line = self.bytecode.get(self.index);
        if let None=line {self.active = false};
        line
    }

    #[inline(always)]
    pub fn eval(&mut self) -> Evaluation {
        if !self.alive {return};
        if let Some(instr)=self.read() {
            Eval::instruction(self, instr)
        }
    }
};

type Evaluation = (bool, Option<String>);

struct Eval {};
impl Eval {

    #[inline(always)]
    fn instruction(
        thread: &mut EulerThread,
        instruction: Instruction
    ) -> Evaluation {
        let (bytecode, arg) = op;
        match command {
            0 => Stack::push(thread, arg),
            1 => Stack::pop(thread),
            2 => Scope::set(thread, arg),
            3 => Scope::get(thread, arg),
            31 => Math::add(thread),
            32 => Math::sub(thread),
            33 => Math::mul(thread),
            34 => Math::div(thread),
            41 => If::start(thread),
            42 => If::end(thread),
            43 => If::else(thread),
            51 => Loop::start(thread),
            52 => Loop::end(thread),
            53 => Loop::break(thread),
            54 => Loop::continue(thread),
            91 => Fun::declare(thread, arg),
            92 => Fun::end(thread),
            93 => Fun::return(thread),
            94 => Fun::call(thread, arg),
            _ => Eval::exit(thread, "invalid instruction"),
        }
    }
};

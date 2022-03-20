
use crate::bytecode::{
    ByteCode,
    Instruction
};

type ParentScope = Box<(usize, ThreadScope)>;

struct ThreadScope {
    parent: Option<ParentScope>,
    names: [isize; 256],
    block: [u8; 128],
    stack: [isize; 32],
    flags: [u8; 8],
    blen: u8,
    slen: u8
};

impl ThreadScope {
    pub fn new(
        parent: Option<ParentScope>
    ) -> Self {
        ThreadScope {
            parent: parent,
            names: [0; 256],
            block: [0; 128],
            stack: [0; 32],
            flags: [0; 8],
            blen: 0,
            slen: 0
        }
    }
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
    pub fn eval(&mut self) -> Self {
        if !self.alive {return};
        if let Some(instr)=self.read() {
            Eval::instruction(self, instr);
        };
        self
    }
};

struct Eval {};
impl Eval {
    #[inline(always)]
    fn instruction(
        thread: &mut EulerThread,
        instruction: Instruction
    ) -> isize {
        let (bytecode, arg) = instruction;
        match bytecode {
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
            91 => Fun::start(thread, arg),
            92 => Fun::end(thread),
            93 => Fun::return(thread),
            94 => Fun::call(thread, arg),
            _ => Eval::exit(thread, "invalid instruction"),
        }
    }

    #[inline(always)]
    fn exit(
        thread: &mut EulerThread,
        message: &str
    ) -> isize {
        thread.active = false;
        thread.error = (thread.index, message);
        1
    }
};

struct Stack {}
impl Stack {
    #[inline(always)]
    fn push(
        thread: &mut EulerThread,
        arg: isize
    ) -> isize {
        thread.scope.slen = (
            thread.scope.slen + 1
        );
        thread.scope.stack[
            thread.scope.slen
        ] = arg;
        arg
    }

    #[inline(always)]
    fn pop(
        thread: &mut EulerThread
    ) -> isize {
        thread.scope.slen = (
            thread.scope.slen - 1
        );
        thread.scope.stack[
            thread.scope.slen + 1
        ]
    }
};

struct Scope {}
impl Scope {
    #[inline(always)]
    fn set(
        thread: &mut EulerThread,
        arg: isize
    ) -> isize {
        thread.scope.names[arg as u8] = (
            Stack.pop(thread)
        );
        arg
    }

    #[inline(always)]
    fn get(
        thread: &mut EulerThread,
        arg: isize
    ) -> isize {
        Stack.push(thread,
            thread.scope.names[arg as u8]
        )
    }
};

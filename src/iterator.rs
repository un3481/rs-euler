
// Modules
use crate::bytecode::{
    ByteCode,
    Operation
};

//##########################################################################################################################

type ParentScope = Box<(usize, Scope)>;

struct Scope {
    parent: Option<ParentScope>,
    names: [isize; 256],
    block: [u8; 32],
    stack: [isize; 8],
    flags: [u8; 4],
    b: u8,
    s: u8
};

impl Scope {

    #[inline(always)]
    pub fn new(
        parent: Option<ParentScope>
    ) -> Self {
        Scope {
            parent: parent,
            names: [0; 256],
            block: [0; 32],
            stack: [0; 8],
            flags: [0; 4],
            b: 0,
            s: 0
        }
    }
};

type ThreadError = (u8, &str);

//##########################################################################################################################

struct EulerThread {
    alive: bool,
    index: usize,
    bytecode: &ByteCode,
    scope: Scope,
    error: Option<ThreadError>
};

impl EulerThread {

    #[inline(always)]
    pub fn new(
        bytecode: &ByteCode,
        index: usize
    ) -> Self {
        EulerThread {
            alive: true,
            index: index,
            bytecode: bytecode,
            scope: Scope::new(None),
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
        if !self.alive {return self};
        if let Some(op)=self.read() { Eval::operation(self, op) };
        self
    }
};

//##########################################################################################################################

struct Eval {};

impl Eval {

    #[inline(always)]
    fn operation(
        thread: &mut EulerThread,
        operation: Operation
    ) -> isize {
        let (bytecode, arg) = operation;
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
            _ => Eval::error(thread, "invalid operation"),
        }
    }
 
    #[inline(always)]
    fn exit(
        thread: &mut EulerThread
    ) -> isize {
        thread.active = false;
        0
    }

    #[inline(always)]
    fn error(
        thread: &mut EulerThread,
        message: &str
    ) -> isize {
        Eval::exit(thread);
        thread.error = (thread.index, message);
        1
    }
};

//##########################################################################################################################

struct Stack {}

impl Stack {

    #[inline(always)]
    fn push(
        thread: &mut EulerThread,
        arg: isize
    ) -> isize {
        thread.scope.s = thread.scope.s + 1;
        thread.scope.stack[thread.scope.s - 1] = arg;
        arg
    }

    #[inline(always)]
    fn pop(
        thread: &mut EulerThread
    ) -> isize {
        thread.scope.s = thread.scope.s - 1;
        thread.scope.stack[thread.scope.s + 1]
    }
};

//##########################################################################################################################

struct Scope {}

impl Scope {

    #[inline(always)]
    fn set(
        thread: &mut EulerThread,
        arg: isize
    ) -> isize {
        thread.scope.names[arg as u8] = Stack::pop(thread);
        thread.scope.names[arg as u8]
    }

    #[inline(always)]
    fn get(
        thread: &mut EulerThread,
        arg: isize
    ) -> isize {
        Stack::push(thread, thread.scope.names[arg as u8])
    }
};

//##########################################################################################################################

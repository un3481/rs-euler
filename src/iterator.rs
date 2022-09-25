
// Modules
use crate::bytecode::{
    Code,
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

    #[inline]
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

pub struct EulerThread {
    alive: bool,
    index: usize,
    bytecode: &ByteCode,
    scope: Scope,
    error: Option<ThreadError>
};

impl EulerThread {

    #[inline]
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

    #[inline]
    pub fn goto(&mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    #[inline] 
    pub fn next(&mut self) -> Self {
        self.goto(self.index + 1)
    }

    #[inline]
    fn read(&self) -> Option<Instruction> {
        self.bytecode.get(self.index)
    }

    #[inline]
    pub fn exit(&mut self) -> Self {
        self.active = false;
        self
    }

    #[inline]
    pub fn raise(&mut self, message: &str) -> isize {
        self.exit();
        self.error = (self.index, message)
    }

    #[inline]
    pub fn eval(&mut self) -> Self {
        if self.alive {
            if let Some(op)=self.read() { Eval::operation(self, op) }
            else { self.exit() }
        };
        self
    }
};

//##########################################################################################################################

struct Eval {};

impl Eval {

    #[inline]
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
             4 => Line::break(thread),
             5 => Bool::and(thread),
             6 => Bool::or(thread),
             7 => Math::add(thread),
             8 => Math::sub(thread),
             9 => Math::mul(thread),
            10 => Math::div(thread),
            11 => If::start(thread, arg),
            12 => If::else(thread, arg),
            13 => Loop::start(thread),
            14 => Loop::continue(thread, arg),
            15 => Loop::break(thread, arg),
            16 => Fun::start(thread, arg),
            17 => Fun::end(thread),
            18 => Fun::call(thread, arg),
            19 => Error::raise(thread),
             _ => Error::inop(thread),
        }
    } 
};

//##########################################################################################################################

struct Error {}

impl Error {

    #[inline]
    pub fn raise(thread: &mut EulerThread) -> isize {
        let message = String::get(thread); // get string from pointer in stack top
        thread.raise(message); // raise error with custom message string
        1
    }

    #[inline]
    pub fn inop(thread: &mut EulerThread) -> isize {
        thread.raise("invalid operation"); // raise error for invalid operations
        1
    }
};

//##########################################################################################################################

struct Line {}

impl Line {

    #[inline]
    pub fn break(thread: &mut EulerThread) -> isize {
        Stack::clear(thread)
    }
};

//##########################################################################################################################

struct Stack {}

impl Stack {

    #[inline]
    pub fn push(thread: &mut EulerThread, arg: isize) -> isize {
        thread.scope.stack[thread.scope.s] = arg; // push value to stack top
        thread.scope.s = thread.scope.s + 1; // add one position to stack
        thread.scope.s as isize // return stack length
    }

    #[inline]
    pub fn pop(thread: &mut EulerThread) -> isize {
        if thread.scope.s == 0 {0}
        else {
            let val = thread.scope.stack[thread.scope.s]; // get stack top
            thread.scope.s = thread.scope.s - 1; // clear one position from stack
            val // return old stack top
        }
    }

    #[inline]
    pub fn clear(thread: &mut EulerThread) -> isize {
        thread.scope.s = 0; // clear all stack positions
        0
    }

    #[inline]
    pub fn reset(thread: &mut EulerThread) -> isize {
        if thread.scope.s == 0 {0}
        else {
            thread.scope.stack[0] = thread.scope.stack[thread.scope.s - 1]; // move stack top to bottom
            thread.scope.s = 1; // clear trailing positions from stack
            thread.scope.stack[0] // return stack top
        }
    }
};

//##########################################################################################################################

struct Scope {}

impl Scope {

    #[inline]
    pub fn set(thread: &mut EulerThread, arg: isize) -> isize {
        thread.scope.names[arg as u8] = Stack::pop(thread); // assign stack top to variable
        thread.scope.names[arg as u8] // return variable value
    }

    #[inline]
    pub fn get(thread: &mut EulerThread, arg: isize) -> isize {
        Stack::push(thread, thread.scope.names[arg as u8]) // push variable to stack top
    }
};

//##########################################################################################################################

struct Bool {}

impl Bool {

    #[inline]
    fn to_int(value: bool) -> isize {
        if (value) {1} else {0}
    }

    #[inline]
    pub fn and(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, to_int((t2 != 0) && (t1 != 0)))
    }

    #[inline]
    pub fn or(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, to_int((t2 != 0) || (t1 != 0)))
    }
};

//##########################################################################################################################

struct Math {}

impl Math {

    #[inline]
    pub fn sum(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 + t1)
    }

    #[inline]
    pub fn sub(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 - t1)
    }

    #[inline]
    pub fn mul(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 * t1)
    }

    #[inline]
    pub fn div(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 / t1)
    }
};

//##########################################################################################################################

struct If {}

impl If {

    #[inline]
    pub fn start(thread: &mut EulerThread, arg: isize) -> isize {
        if Stack::pop(thread) == 0 { thread.goto(arg as usize) }; // if stack top is 0 go to [if_else + 1]
        Stack::clear(thread)
    }

    #[inline]
    pub fn else(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [if_end]
        Stack::clear(thread)
    }
};

//##########################################################################################################################

struct Loop {}

impl Loop {

    #[inline]
    pub fn start(thread: &mut EulerThread) -> isize {
        Stack::clear(thread)
    }

    #[inline]
    pub fn end(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [loop_start]
        0
    }

    #[inline]
    pub fn break(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [loop_end + 1]
        0
    }
};

//##########################################################################################################################

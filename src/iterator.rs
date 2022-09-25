
// Modules
use crate::bytecode::{
    Code,
    ByteCode,
    Operation
};

//##########################################################################################################################

type ParentScope = Box<(usize, Scope)>;

struct Scope {
    pub parent: Option<ParentScope>,
    pub names: [isize; 256],
    pub stack: [isize; 8],
    pub flags: [u8; 4],
    pub b: u8,
    pub s: u8
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
    pub bytecode: &ByteCode,
    pub error: Option<ThreadError>,
    alive: bool,
    index: usize,
    scope: Scope
};

impl EulerThread {

    #[inline]
    pub fn new(
        bytecode: &ByteCode,
        index: usize
    ) -> Self {
        EulerThread {
            bytecode: bytecode,
            error: None,
            alive: true,
            index: index,
            scope: Scope::new(None)
        }
    }

    #[inline]
    pub fn fold(&mut self) -> isize {
        self.scope = Scope::new((self.index, self.scope));
        self.index = 0;
        0
    }

    #[inline]
    pub fn unfold(&mut self) -> isize {
        if let Some(parent) = self.scope.parent {
            let (index, scope) = parent;
            self.index = index;
            self.scope = scope;
            0
        } else {1}
    }
};

//##########################################################################################################################

impl EulerThread {

    #[inline]
    pub fn goto(&mut self, index: usize) -> isize {
        self.index = index;
        0
    }

    #[inline] 
    pub fn next(&mut self) -> isize {
        self.goto(self.index + 1)
    }

    #[inline]
    fn read(&self) -> Option<Instruction> {
        self.bytecode.get(self.index)
    }

    #[inline]
    pub fn exit(&mut self) -> isize {
        self.active = false;
        0
    }

    #[inline]
    pub fn raise(&mut self, message: &str) -> isize {
        self.exit();
        self.error = (self.index, message);
        0
    }

    #[inline]
    pub fn eval(&mut self) -> isize {
        if !self.alive {1}
        else {
            if let Some(op)=self.read() {
                Eval::operation(self, op);
                0
            }
            else {
                self.exit();
                1
            }
        }
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
             0 => Stack::push(thread, arg) * thread.next(),
             1 => Stack::pop(thread)       * thread.next(),
             2 => Scope::set(thread, arg)  * thread.next(),
             3 => Scope::get(thread, arg)  * thread.next(),
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
        Stack::clear(thread);
        thread.next()
    }
};

//##########################################################################################################################

struct Stack {}

impl Stack {

    #[inline]
    pub fn push(thread: &mut EulerThread, arg: isize) -> isize {
        thread.scope.stack[thread.scope.s] = arg; // push value to top of stack
        thread.scope.s = thread.scope.s + 1; // add one position to stack
        thread.scope.s as isize // return stack length
    }

    #[inline]
    pub fn pop(thread: &mut EulerThread) -> isize {
        if thread.scope.s == 0 {0}
        else {
            let val = thread.scope.stack[thread.scope.s]; // get top of stack
            thread.scope.s = thread.scope.s - 1; // clear one position from stack
            val // return top of stack
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
            thread.scope.stack[0] = thread.scope.stack[thread.scope.s - 1]; // move top of stack to bottom
            thread.scope.s = 1; // clear trailing positions from stack
            thread.scope.stack[0] // return top of stack
        }
    }
};

//##########################################################################################################################

struct Scope {}

impl Scope {

    #[inline]
    pub fn set(thread: &mut EulerThread, arg: isize) -> isize {
        thread.scope.names[arg as u8] = Stack::pop(thread); // assign top of stack to variable
        thread.scope.names[arg as u8] // return variable value
    }

    #[inline]
    pub fn get(thread: &mut EulerThread, arg: isize) -> isize {
        Stack::push(thread, thread.scope.names[arg as u8]) // push variable to top of stack
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
        Stack::push(thread, to_int((t2 != 0) && (t1 != 0)));
        thread.next()
    }

    #[inline]
    pub fn or(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, to_int((t2 != 0) || (t1 != 0)));
        thread.next()
    }
};

//##########################################################################################################################

struct Math {}

impl Math {

    #[inline]
    pub fn sum(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 + t1);
        thread.next()
    }

    #[inline]
    pub fn sub(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 - t1);
        thread.next()
    }

    #[inline]
    pub fn mul(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 * t1);
        thread.next()
    }

    #[inline]
    pub fn div(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, t2 / t1);
        thread.next()
    }
};

//##########################################################################################################################

struct If {}

impl If {

    #[inline]
    pub fn start(thread: &mut EulerThread, arg: isize) -> isize {
        if Stack::pop(thread) != 0 { thread.next() } // if top of stack is not 0 go to [next]
        else { thread.goto(arg as usize) }; // if top of stack is 0 go to [else + 1]
        Stack::clear(thread)
    }

    #[inline]
    pub fn else(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [end + 1]
        Stack::clear(thread)
    }
};

//##########################################################################################################################

struct Loop {}

impl Loop {

    #[inline]
    pub fn start(thread: &mut EulerThread) -> isize {
        Stack::clear(thread);
        thread.next()
    }

    #[inline]
    pub fn end(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [start]
    }

    #[inline]
    pub fn break(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [end + 1]
    }
};

//##########################################################################################################################

struct Fun {}

impl Fun {

    #[inline]
    pub fn start(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [end + 1]
    }

    #[inline]
    pub fn end(thread: &mut EulerThread) -> isize {
        let val = Stack::pop(thread); // get top of stack
        if thread.unfold() != 0 {
            thread.raise("return called with no parent scope");
            1
        } else {
            Stack::push(thread, val) // push function return to top of stack
            thread.next() // go to [next]
            0
        }
    }

    #[inline]
    pub fn call(thread: &mut EulerThread, arg: isize) -> isize {
        thread.fold(); // crate new scope
        thread.goto(arg as usize); // go to [start + 1]
        0
    }
};

//##########################################################################################################################

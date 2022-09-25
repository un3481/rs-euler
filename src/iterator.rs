
// Imports
use std::mem::swap;
use std::sync::mpsc;

// Modules
use crate::bytecode::{ ByteCode, Operation };
use crate::types::{ EulerString };

//##########################################################################################################################

type Shared = (usize, usize, isize);

pub struct Channel {
    tx: mpsc::Sender<Shared>,
    rx: mpsc::Receiver<Shared>
}

//##########################################################################################################################

type ParentScope = Box<(usize, ThreadScope)>;

struct ThreadScope {
    pub parent: Option<ParentScope>,
    pub names: [isize; 256],
    pub stack: [isize; 8],
    pub s: u8
}

impl ThreadScope {

    #[inline]
    pub fn new(
        parent: Option<ParentScope>
    ) -> Self {
        ThreadScope {
            parent: parent,
            names: [0; 256],
            stack: [0; 8],
            s: 0
        }
    }
}

type ThreadError = (u8, String);

//##########################################################################################################################

pub struct EulerThread<'a> {
    pub pid: usize,
    pub scheduler: Channel,
    index: usize,
    bytecode: &'a ByteCode,
    error: Option<ThreadError>,
    alive: bool,
    wait: bool,
    scope: ThreadScope
}

impl<'a> EulerThread<'a> {

    #[inline]
    pub fn new(
        pid: usize,
        scheduler: Channel,
        index: usize,
        bytecode: &'a ByteCode
    ) -> Self {
        EulerThread::<'a> {
            pid: pid,
            scheduler: scheduler,
            index: index,
            bytecode: bytecode,
            error: None,
            alive: true,
            wait: false,
            scope: ThreadScope::new(None)
        }
    }
}

//##########################################################################################################################

impl EulerThread<'_> {

    #[inline]
    pub fn fold(&mut self) -> isize {
        let mut scope = ThreadScope::new(None);
        swap(&mut scope, &mut self.scope); // extract scope from borrow
        let stack = scope.stack; // store stack
        self.scope = ThreadScope::new(
            Some(Box::new((self.index, scope)))
        );
        self.scope.stack = stack; // assign stack
        self.index = 0;
        0
    }

    #[inline]
    pub fn unfold(&mut self) -> isize {
        let mut opt_parent: Option<ParentScope> = None;
        swap(&mut opt_parent, &mut self.scope.parent);  // Extract parent scope from borrow
        if let Some(parent) = opt_parent {
            let (index, scope) = *parent;
            self.scope = scope;
            self.index = index;
            0
        } else {1}
    }
}

//##########################################################################################################################

impl<'a> EulerThread<'a> {

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
    pub fn exit(&mut self) -> isize {
        self.alive = false;
        0
    }

    #[inline]
    pub fn raise(&mut self, message: String) -> isize {
        self.exit();
        self.error = Some(
            (self.index as u8, message)
        );
        0
    }

    #[inline]
    pub fn eval(&mut self) -> isize {
        if !self.alive {1}
        else {
            if let Some(op)=self.bytecode.get(self.index) {
                Eval::operation(self, op);
                0
            } else {
                self.exit();
                1
            }
        }
    }
}

//##########################################################################################################################

struct Eval {}

impl Eval {

    #[inline]
    fn operation(
        thread: &mut EulerThread,
        operation: &Operation
    ) -> isize {
        let (bytecode, arg) = operation;
        match bytecode {
             0 => Stack::push(thread, *arg) * thread.next(),
             1 => Stack::pop(thread)        * thread.next(),
             2 => Scope::set(thread, *arg)  * thread.next(),
             3 => Scope::get(thread, *arg)  * thread.next(),
             4 => Line::r#break(thread),
             5 => Bool::and(thread),
             6 => Bool::or(thread),
             7 => Math::add(thread),
             8 => Math::sub(thread),
             9 => Math::mul(thread),
            10 => Math::div(thread),
            11 => If::start(thread, *arg),
            12 => If::r#else(thread, *arg),
            13 => Loop::start(thread),
            14 => Loop::end(thread, *arg),
            15 => Loop::r#break(thread, *arg),
            16 => Fun::start(thread, *arg),
            17 => Fun::end(thread),
            18 => Fun::call(thread, *arg),
            19 => Fun::apply(thread),
            20 => Scheduler::pid(thread),
            21 => Scheduler::spawn(thread, *arg),
            22 => Scheduler::send(thread),
            23 => Scheduler::receive(thread),
            24 => Error::raise(thread),
             _ => Error::inop(thread),
        }
    }
}

//##########################################################################################################################

struct Error {}

impl Error {

    #[inline]
    pub fn raise(thread: &mut EulerThread) -> isize {
        let top = Stack::pop(thread); // get string pointer in stack top
        thread.raise(
            EulerString::get(top)
        );
        1
    }

    #[inline]
    pub fn inop(thread: &mut EulerThread) -> isize {
        thread.raise(
            String::from("invalid operation")
        );
        1
    }
}

//##########################################################################################################################

struct Line {}

impl Line {

    #[inline]
    pub fn r#break(thread: &mut EulerThread) -> isize {
        Stack::clear(thread);
        thread.next()
    }
}

//##########################################################################################################################

struct Stack {}

impl Stack {

    #[inline]
    pub fn push(thread: &mut EulerThread, arg: isize) -> isize {
        thread.scope.stack[thread.scope.s as usize] = arg; // push value to top of stack
        thread.scope.s = thread.scope.s + 1; // add one position to stack
        thread.scope.s as isize // return stack length
    }

    #[inline]
    pub fn pop(thread: &mut EulerThread) -> isize {
        if thread.scope.s == 0 {0}
        else {
            let val = thread.scope.stack[thread.scope.s as usize]; // get top of stack
            thread.scope.s = thread.scope.s - 1; // clear one position from stack
            val // return top of stack
        }
    }

    #[inline]
    pub fn clear(thread: &mut EulerThread) -> isize {
        thread.scope.s = 0; // clear all stack positions
        0
    }
}

//##########################################################################################################################

struct Scope {}

impl Scope {

    #[inline]
    pub fn set(thread: &mut EulerThread, arg: isize) -> isize {
        thread.scope.names[arg as usize] = Stack::pop(thread); // assign top of stack to variable
        thread.scope.names[arg as usize] // return variable value
    }

    #[inline]
    pub fn get(thread: &mut EulerThread, arg: isize) -> isize {
        Stack::push(thread, thread.scope.names[arg as usize]) // push variable to top of stack
    }
}

//##########################################################################################################################

struct Bool {}

impl Bool {

    #[inline]
    pub fn and(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, if (t2 != 0) && (t1 != 0) {1} else {0});
        thread.next()
    }

    #[inline]
    pub fn or(thread: &mut EulerThread) -> isize {
        let t1 = Stack::pop(thread);
        let t2 = Stack::pop(thread);
        Stack::push(thread, if (t2 != 0) || (t1 != 0) {1} else {0});
        thread.next()
    }
}

//##########################################################################################################################

struct Math {}

impl Math {

    #[inline]
    pub fn add(thread: &mut EulerThread) -> isize {
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
}

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
    pub fn r#else(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize); // go to [end + 1]
        Stack::clear(thread)
    }
}

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
        thread.goto(arg as usize) // go to [start]
    }

    #[inline]
    pub fn r#break(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize) // go to [end + 1]
    }
}

//##########################################################################################################################

struct Fun {}

impl Fun {

    #[inline]
    pub fn start(thread: &mut EulerThread, arg: isize) -> isize {
        thread.goto(arg as usize) // go to [end + 1]
    }

    #[inline]
    pub fn end(thread: &mut EulerThread) -> isize {
        let top = Stack::pop(thread); // get top of stack
        if thread.unfold() == 0 {
            Stack::push(thread, top); // push function return to top of stack
            thread.next() // go to [next]
        } else {
            thread.raise(
                String::from("return called with no parent scope")
            );
            1
        }
    }

    #[inline]
    pub fn call(thread: &mut EulerThread, arg: isize) -> isize {
        thread.fold(); // crate new scope
        thread.goto(arg as usize); // go to [start + 1]
        0
    }

    #[inline]
    pub fn apply(thread: &mut EulerThread) -> isize {
        thread.fold(); // crate new scope
        let top = Stack::pop(thread);
        thread.goto(top as usize); // go to [start + 1]
        0
    }
}

//##########################################################################################################################

struct Scheduler {}

impl Scheduler {

    #[inline]
    pub fn pid(thread: &mut EulerThread) -> isize {
        Stack::push(thread, thread.pid as isize);
        thread.next()
    }

    #[inline]
    pub fn spawn(thread: &mut EulerThread, arg: isize) -> isize {
        if !thread.wait {
            thread.scheduler.tx.send(
                (thread.pid, 0, arg)
            ).unwrap();
        };
        thread.wait = true;
        match thread.scheduler.rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => 0,
            Err(mpsc::TryRecvError::Disconnected) => {
                Stack::push(thread, -1);
                thread.wait = false;
                thread.next()
            },
            Ok((0, _, pid)) => {
                Stack::push(thread, pid);
                thread.wait = false;
                thread.next()
            },
            Ok(_) => {
                Stack::push(thread, -1);
                thread.wait = false;
                thread.next()
            },
        }
    }

    #[inline]
    pub fn send(thread: &mut EulerThread) -> isize {
        let val = Stack::pop(thread);
        let pid = Stack::pop(thread);
        thread.scheduler.tx.send(
            (thread.pid, pid as usize, val)
        ).unwrap();
        thread.next()
    }

    #[inline]
    pub fn receive(thread: &mut EulerThread) -> isize {
        thread.wait = true;
        match thread.scheduler.rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => 0,
            Err(mpsc::TryRecvError::Disconnected) => {
                Stack::push(thread, 1);
                thread.wait = false;
                thread.next()
            },
            Ok((_, _, val)) => {
                Stack::push(thread, val);
                Stack::push(thread, 0);
                thread.wait = false;
                thread.next()
            },
        }
    }
}

//##########################################################################################################################

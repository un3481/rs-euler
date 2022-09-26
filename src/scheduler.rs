
// Imports
use std::thread;
use std::collections::HashMap;

// Modules
use crate::iterator::{ EulerThread };

//##########################################################################################################################

type ThreadHash<'a> = HashMap<usize, EulerThread<'a>>;

struct OperationBlock<'a> {
    hash: ThreadHash<'a>,
    lts: usize
}

impl<'a> OperationBlock<'a> {

    #[inline]
    pub fn new() -> Self {
        OperationBlock::<'a> {
            hash: HashMap::new(),
            lts: 0
        }
    }

    #[inline]
    pub fn insert(&mut self, euler_thread: EulerThread<'a>) -> () {
        self.hash.insert(self.lts, euler_thread);
        self.lts = self.lts + 1;
    }
}

//##########################################################################################################################

pub struct EulerScheduler {
    scheduler: thread::Thread,
    pool: rayon::ThreadPool,
}

impl EulerScheduler {

    #[inline]
    pub fn new() -> Self {
        let cpus = num_cpus::get();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cpus)
            .build()
            .unwrap();
        let scheduler = thread::spawn(move || {
            Self::execute(pool)
        });
        EulerScheduler {
            scheduler: scheduler,
            pool: pool
        }
    }
}

//##########################################################################################################################

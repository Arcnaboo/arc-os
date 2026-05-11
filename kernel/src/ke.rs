use core::sync::atomic::{AtomicUsize, Ordering};
// NT defines IRQL (Interrupt Request Level)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Irql {
    PassiveLevel = 0,
    ApcLevel = 1,
    DispatchLevel = 2,
    DeviceLevel = 3,
    HighLevel = 15,
}

static CURRENT_IRQL: AtomicUsize = AtomicUsize::new(Irql::PassiveLevel as usize);

pub fn ke_get_current_irql() -> Irql {
    let irql = CURRENT_IRQL.load(Ordering::SeqCst) as u8;
    unsafe { core::mem::transmute(irql) }
}

pub fn ke_raise_irql(new_irql: Irql, old_irql: &mut Irql) {
    let current = ke_get_current_irql();
    if new_irql > current {
        *old_irql = current;
        CURRENT_IRQL.store(new_irql as usize, Ordering::SeqCst);
    }
}

pub fn ke_lower_irql(new_irql: Irql) {
    CURRENT_IRQL.store(new_irql as usize, Ordering::SeqCst);
}

#[derive(Debug)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}

// Basic NT Object Header
#[derive(Debug)]
pub struct DispatcherHeader {
    pub type_code: u8,
    pub absolute: u8,
    pub size: u8,
    pub inserted: u8,
    pub signal_state: i32,
    pub wait_list_head: ListEntry, // Replaced Vec with ListEntry
}

// Thread State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Initialized,
    Ready,
    Running,
    Standby,
    Terminated,
    Waiting,
    Transition,
}

use spin::Mutex;

// KTHREAD (Kernel Thread)
#[derive(Debug)]
pub struct KThread {
    pub header: DispatcherHeader,
    pub state: ThreadState,
    pub priority: i8,
    pub base_priority: i8,
    pub affinity: usize,
    pub user_time: u32,
    pub kernel_time: u32,
    pub stack_base: usize,
    pub stack_limit: usize,
    pub kernel_stack: usize,
    // process: *mut KProcess
}

impl KThread {
    pub fn new() -> Self {
        Self {
            header: DispatcherHeader {
                type_code: 6, // ThreadObject
                absolute: 0,
                size: core::mem::size_of::<Self>() as u8,
                inserted: 0,
                signal_state: 0,
                wait_list_head: ListEntry {
                    flink: core::ptr::null_mut(),
                    blink: core::ptr::null_mut(),
                },
            },
            state: ThreadState::Initialized,
            priority: 8, // Normal priority
            base_priority: 8,
            affinity: !0,
            user_time: 0,
            kernel_time: 0,
            stack_base: 0,
            stack_limit: 0,
            kernel_stack: 0,
        }
    }
}

// Global Scheduler lock
pub static DISPATCHER_LOCK: Mutex<()> = Mutex::new(());

// Basic KeInitializeThread function
pub fn ke_initialize_thread(thread: &mut KThread) {
    let _lock = DISPATCHER_LOCK.lock();
    thread.state = ThreadState::Initialized;
    crate::println!("Thread initialized with priority {}", thread.priority);
}

// APC (Asynchronous Procedure Call)
#[derive(Debug)]
pub struct Kapc {
    pub type_code: u8,
    pub size: u8,
    pub thread: *mut KThread,
    pub apc_state_index: u8,
    pub processor_mode: u8,
    pub inserted: bool,
}

impl Kapc {
    pub fn new(thread: *mut KThread) -> Self {
        Self {
            type_code: 18, // ApcObject
            size: core::mem::size_of::<Self>() as u8,
            thread,
            apc_state_index: 0,
            processor_mode: 0, // KernelMode
            inserted: false,
        }
    }
}

// DPC (Deferred Procedure Call)
#[derive(Debug)]
pub struct Kdpc {
    pub type_code: u8,
    pub importance: u8,
    pub number: u16,
    pub deferred_routine: usize, // Pointer to function
    pub deferred_context: usize,
    pub system_argument1: usize,
    pub system_argument2: usize,
}

impl Kdpc {
    pub fn new(routine: usize, context: usize) -> Self {
        Self {
            type_code: 19, // DpcObject
            importance: 1, // MediumImportance
            number: 0,
            deferred_routine: routine,
            deferred_context: context,
            system_argument1: 0,
            system_argument2: 0,
        }
    }
}

use alloc::boxed::Box;
use alloc::sync::Arc;
use spin::Mutex;
use core::cell::UnsafeCell;
use alloc::vec::Vec;

struct Process {
    id: Pid,
    status: Status,
    status_after_stop: Status,
    context: Option<Box<Context>>,
}

pub type Pid = usize;
type ExitCode = usize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    Ready,
    Running(usize),
    Sleeping,
    Waiting(Pid),
    /// aka ZOMBIE. Its context was dropped.
    Exited(ExitCode),
}

#[derive(Eq, PartialEq)]
enum Event {
    Wakeup(Pid),
}

pub trait Context {
    unsafe fn switch_to(&mut self, target: &mut Context);
}

pub struct ProcessManager {
    procs: Vec<Mutex<Option<Process>>>,
    scheduler: Mutex<Vec<Pid>>,
}

impl ProcessManager {
    pub fn new(max_proc_num: usize) -> Self {
        ProcessManager {
            procs: new_vec_default(max_proc_num),
            scheduler: Mutex::new(new_vec_default(0)),
        }
    }

    fn alloc_pid(&self) -> Pid {
        for (i, proc_mux) in self.procs.iter().enumerate() {
            let mut proc_lock = proc_mux.lock();
            if proc_lock.is_none() {
                return i;
            }
            let proc = proc_lock.as_ref().expect("process not exist");
        }
        panic!("Process number exceeded");
    }

    /// Add a new process
    pub fn add(&self, context: Box<Context>, parent: Pid) -> Pid {
        let pid = self.alloc_pid();
        *(&self.procs[pid]).lock() = Some(Process {
            id: pid,
            status: Status::Ready,
            status_after_stop: Status::Ready,
            context: Some(context),
        });
        self.scheduler.lock().insert(0, pid);
        debug!("add process {}", pid);
        pid
    }

    /// Make process `pid` time slice -= 1.
    /// Return true if time slice == 0.
    /// Called by timer interrupt handler.
    pub fn tick(&self, pid: Pid) -> bool {
        false
    }

    /// Set the priority of process `pid`
    pub fn set_priority(&self, pid: Pid, priority: u8) {

    }

    /// Called by Processor to get a process to run.
    /// The manager first mark it `Running`,
    /// then take out and return its Context.
    pub fn run(&self, cpu_id: usize) -> (Pid, Box<Context>) {
        let mut scheduler = self.scheduler.lock();
        let pid = scheduler.pop().unwrap();
        let mut proc_lock = self.procs[pid].lock();
        let mut proc = proc_lock.as_mut().expect("process not exist");
        proc.status = Status::Running(cpu_id);
        (pid, proc.context.take().expect("context not exist"))
    }

    /// Called by Processor to finish running a process
    /// and give its context back.
    pub fn stop(&self, pid: Pid, context: Box<Context>) {
        let mut proc_lock = self.procs[pid].lock();
        let mut proc = proc_lock.as_mut().expect("process not exist");
        proc.status = proc.status_after_stop.clone();
        proc.status_after_stop = Status::Ready;
        proc.context = Some(context);
        match proc.status {
            Status::Ready => self.scheduler.lock().insert(0, pid),
            Status::Exited(_) => self.exit_handler(pid, proc),
            _ => {}
        }
    }

    /// Switch the status of a process.
    /// Insert/Remove it to/from scheduler if necessary.
    fn set_status(&self, pid: Pid, status: Status) {
        unimplemented!()
    }

    pub fn get_status(&self, pid: Pid) -> Option<Status> {
        unimplemented!()
    }

    /// Remove an exited proc `pid`.
    /// Its all children will be set parent to 0.
    pub fn remove(&self, pid: Pid) {
        unimplemented!()
    }

    /// Sleep `pid` for `time` ticks.
    /// `time` == 0 means sleep forever
    pub fn sleep(&self, pid: Pid, time: usize) {
        unimplemented!()
    }

    pub fn wakeup(&self, pid: Pid) {
        unimplemented!()
    }

    pub fn wait(&self, pid: Pid, target: Pid) {
        unimplemented!()
    }
    pub fn wait_child(&self, pid: Pid) {
        unimplemented!()
    }

    pub fn get_children(&self, pid: Pid) -> Vec<Pid> {
        unimplemented!()
    }

    pub fn exit(&self, pid: Pid, code: ExitCode) {
        debug!("{} exit", pid);
        let mut proc_lock = self.procs[pid].lock();
        let mut proc = proc_lock.as_mut().expect("process not exist");
        proc.status = Status::Exited(code);
        self.scheduler.lock().retain(|&i| i != pid);
        proc.status_after_stop = Status::Exited(code);
        self.exit_handler(pid, proc)
    }
    /// Called when a process exit
    fn exit_handler(&self, pid: Pid, proc: &mut Process) {
        proc.context = None;
    }
}


fn new_vec_default<T: Default>(size: usize) -> Vec<T> {
    let mut vec = Vec::new();
    vec.resize_default(size);
    vec
}

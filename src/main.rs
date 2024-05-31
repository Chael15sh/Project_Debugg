use std::process::{Command, Child};
use std::io::{self, Write};
use nix::unistd::Pid;
use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::sys::signal::Signal;
use gimli::{EndianReader, RunTimeEndian, EhFrame, BaseAddresses, UnwindContext, UnwindSection};

fn main() {
    let mut child = None;

    loop {
        let input = get_user_input();
        match input.as_str() {
            "run" => {
                if child.is_some() {
                    kill_child(&mut child);
                }
                child = Some(spawn_child());
            }
            "continue" => {
                if let Some(ref mut c) = child {
                    ptrace::cont(Pid::from_raw(c.id() as i32), None)
                        .expect("Failed to continue child process");
                } else {
                    println!("Nothing is being debugged!");
                }
            }
            "quit" => {
                kill_child(&mut child);
                break;
            }
            _ => {
                println!("Unknown command: {}", input);
            }
        }

        if let Some(ref mut c) = child {
            match wait_for_signal(c) {
                Some(signal_num) => {
                    if signal_num == Signal::SIGTRAP as i32 {
                        println!("Breakpoint hit!");
                        handle_breakpoint(c);
                    } else {
                        let signal = signal_from_num(signal_num);
                        println!("Signal received: {:?}", signal);
                    }
                }
                None => {
                    println!("Child process exited");
                    child = None;
                }
            }
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    print!("(deet) ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn spawn_child() -> Child {
    let child = Command::new("./samples/sleepy_print")
        .arg("5")
        .spawn()
        .expect("Failed to spawn child process");

    ptrace::attach(Pid::from_raw(child.id() as i32))
        .expect("Failed to attach to child process");

    child
}

fn wait_for_signal(child: &mut Child) -> Option<i32> {
    let pid = Pid::from_raw(child.id() as i32);
    let wait_result = waitpid(pid, Some(WaitPidFlag::WSTOPPED));

    match wait_result {
        Ok(wait_status) => {
            if let WaitStatus::Stopped(_, signal) = wait_status {
                Some(signal as i32)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn signal_from_num(signal_num: i32) -> Signal {
    match signal_num {
        1 => Signal::SIGHUP,
        2 => Signal::SIGINT,
        3 => Signal::SIGQUIT,
        4 => Signal::SIGILL,
        5 => Signal::SIGTRAP,
        6 => Signal::SIGABRT,
        7 => Signal::SIGBUS,
        8 => Signal::SIGFPE,
        9 => Signal::SIGKILL,
        10 => Signal::SIGUSR1,
        11 => Signal::SIGSEGV,
        12 => Signal::SIGUSR2,
        13 => Signal::SIGPIPE,
        14 => Signal::SIGALRM,
        15 => Signal::SIGTERM,
        16 => Signal::SIGSTKFLT,
        17 => Signal::SIGCHLD,
        18 => Signal::SIGCONT,
        19 => Signal::SIGSTOP,
        20 => Signal::SIGTSTP,
        21 => Signal::SIGTTIN,
        22 => Signal::SIGTTOU,
        23 => Signal::SIGURG,
        24 => Signal::SIGXCPU,
        25 => Signal::SIGXFSZ,
        26 => Signal::SIGVTALRM,
        27 => Signal::SIGPROF,
        28 => Signal::SIGWINCH,
        29 => Signal::SIGIO,
        30 => Signal::SIGPWR,
        31 => Signal::SIGSYS,
        _ => Signal::SIGTERM,
    }
}

fn handle_breakpoint(child: &mut Child) {
    print_backtrace(child);
    step(child);
    print_registers(child);
}

fn print_backtrace(child: &mut Child) {
    let pid = Pid::from_raw(child.id() as i32);
    let binary_data: &[u8] = &[0x01, 0x02, 0x03, 0x04]; // Remplacez par vos données binaires réelles
    let mut reader = EndianReader::new(binary_data, RunTimeEndian::Little);
    let bases = BaseAddresses::default();
    let mut ctx = UnwindContext::new();

    let eh_frame = EhFrame::new(&mut reader, RunTimeEndian::Little);
    let eh_frame_iter = UnwindSection::entries(&eh_frame, &bases);

    let mut entries = Vec::new();
    for entry in eh_frame_iter {
        match entry {
            Ok(entry) => entries.push(entry),
            Err(err) => println!("Error: {:?}", err),
        }
    }

    for entry in entries.drain(..) {
        let function_name = entry.function_name().unwrap_or("<unknown>");
        let start_address = entry.start_address();
        let end_address = entry.end_address();

        println!("{:#x} - {:#x} : {}", start_address, end_address, function_name);
    }
}

fn step(child: &mut Child) {
    let pid = Pid::from_raw(child.id() as i32);
    ptrace::step(pid, None).expect("Failed to step child process");
}

fn print_registers(child: &mut Child) {
    let pid = Pid::from_raw(child.id() as i32);
    let regs = ptrace::getregs(pid).expect("Failed to get registers");

    println!("Registers:");
    println!("  rax: {:#x}", regs.rax);
    println!("  rbx: {:#x}", regs.rbx);
    println!("  rcx: {:#x}", regs.rcx);
    println!("  rdx: {:#x}", regs.rdx);
    println!("  rsi: {:#x}", regs.rsi);
    println!("  rdi: {:#x}", regs.rdi);
    println!("  rbp: {:#x}", regs.rbp);
    println!("  rsp: {:#x}", regs.rsp);
    println!("  rip: {:#x}", regs.rip);
    println!("  eflags: {:#x}", regs.eflags);
}

fn kill_child(child: &mut Option<Child>) {
    if let Some(mut c) = child.take() {
        let pid = Pid::from_raw(c.id() as i32);
        println!("Killing running inferior (pid {})", pid);
        c.kill().expect("Failed to kill child process");
        let _ = waitpid(pid, None);
    }
}

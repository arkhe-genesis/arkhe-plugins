#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod memory;
mod scheduler;
mod syscalls;
mod ipc;
mod isolation;
mod temporal;

use core::panic::PanicInfo;

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    // Inicializar o kernel

    // Iniciar gerenciador de memória
    memory::init();

    // Iniciar escalonador
    scheduler::init();

    // Configurar isolamento (LVD/MicroVM)
    isolation::init();

    // IPC
    ipc::init();

    // TemporalChain
    temporal::init();

    // Tabela de syscalls
    syscalls::handle_syscall(syscalls::Syscall::FairMetrics);

    // Loop principal (idle task)
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
fn main() {}

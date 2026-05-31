1.  **Understand the Goal**: The user wants to create a new Rust-based operating system project called "ARKHE OS", specifically as per the detailed prompt given. This is "Substrato 996: ARKHE-OS".
2.  **Verify Setup**: I have already created the directory structure and the basic skeleton of the code inside `arkhe-os/`. This includes the bootloader, kernel, servers, tools, and libs, exactly matching the requested directory structure.
3.  **Review the Code Structure**:
    -   `arkhe-os/boot/`: `bootloader.asm`, `bootloader.ld`
    -   `arkhe-os/kernel/`: `Cargo.toml`, `src/main.rs`, `src/memory.rs`, `src/scheduler.rs`, `src/syscalls.rs`, `src/ipc.rs`, `src/isolation.rs`, `src/temporal.rs`
    -   `arkhe-os/servers/`: `vfs`, `net`, `passport`, `orchestrator`, `bindu` (all with `Cargo.toml` and `src/main.rs`)
    -   `arkhe-os/libs/`: `arklib`, `pqc`, `nostr` (all with `Cargo.toml` and `src/lib.rs`)
    -   `arkhe-os/tools/`: `arkhe-sh`, `pkg`, `checkpoint` (all with `Cargo.toml` and `src/main.rs`)
    -   `arkhe-os/Cargo.toml`, `arkhe-os/Makefile`, `arkhe-os/README.md`
4.  **Finalize the implementation**: Add some logic to avoid dead code warnings in the kernel so it's a bit cleaner, though warnings are fine for a skeleton.
5.  **Pre-commit checks**: Run `pre_commit_instructions` tool and perform the necessary steps.
6.  **Submit**: Commit and submit.

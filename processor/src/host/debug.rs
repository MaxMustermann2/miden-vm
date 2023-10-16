use super::ProcessState;
use crate::Vec;
use vm_core::{DebugOptions, Word};

// DEBUG HANDLER
// ================================================================================================

/// Prints the info about the VM state specified by the provided options to stdout.
pub fn print_debug_info<S: ProcessState>(process: &S, options: &DebugOptions) {
    let printer = Printer::new(process.clk(), process.ctx(), process.fmp());
    match options {
        DebugOptions::StackAll => {
            printer.print_vm_stack(process, None);
        }
        DebugOptions::StackTop(n) => {
            printer.print_vm_stack(process, Some(*n as usize));
        }
        DebugOptions::MemAll => {
            printer.print_mem_all(process);
        }
        DebugOptions::MemInterval(n, m) => {
            printer.print_mem_interval(process, *n, *m);
        }
        DebugOptions::LocalInterval((n, m), num_locals, print_all) => {
            printer.print_local_interval(process, (*n, *m), *num_locals, *print_all);
        }
        DebugOptions::All(num_locals) => {
            // print stack
            printer.print_vm_stack(process, None);

            // print memory
            printer.print_mem_all(process);

            // print locals
            printer.print_local_interval(process, (0, 0), *num_locals - 1, true);
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

struct Printer {
    clk: u32,
    ctx: u32,
    fmp: u32,
}

impl Printer {
    fn new(clk: u32, ctx: u32, fmp: u64) -> Self {
        Self {
            clk,
            ctx,
            fmp: fmp as u32,
        }
    }

    fn print_vm_stack<S: ProcessState>(&self, process: &S, n: Option<usize>) {
        let stack = process.get_stack_state();

        // determine how many items to print out
        let num_items = core::cmp::min(stack.len(), n.unwrap_or(stack.len()));

        // print all items except for the last one
        println!("Stack state before step {}:", self.clk);
        for (i, element) in stack.iter().take(num_items - 1).enumerate() {
            println!("├── {i:>2}: {element}");
        }

        // print the last item, and in case the stack has more items, print the total number of
        // un-printed items
        let i = num_items - 1;
        if num_items == stack.len() {
            println!("└── {i:>2}: {}\n", stack[i]);
        } else {
            println!("├── {i:>2}: {}", stack[i]);
            println!("└── ({} more items)\n", stack.len() - num_items);
        }
    }

    /// Prints the whole memory state at the cycle `clk` in context `ctx`.
    fn print_mem_all<S: ProcessState>(&self, process: &S) {
        let mem = process.get_mem_state(self.ctx);

        println!("Memory state before step {} for the context {}:", self.clk, self.ctx);

        // print the main part of the memory (wihtout the last value)
        for (addr, value) in mem.iter().take(mem.len() - 1) {
            print_word(*addr as u32, value, false);
        }

        // print the last memory value
        if let Some((addr, value)) = mem.last() {
            print_word(*addr as u32, value, true);
        } else {
            println!("└── EMPTY\n");
        }
    }

    /// Prints memory values in the provided addresses interval.
    ///
    /// `header` contains a title message corresponding to the DebugOption it is used in
    /// ([DebugOptions::MemInterval] or [DebugOptions::LocalInterval]).
    fn print_mem_interval<S: ProcessState>(&self, process: &S, n: u32, m: u32) {
        let mut mem_interval = Vec::new();
        for addr in n..m + 1 {
            mem_interval.push((addr, process.get_mem_value(self.ctx, addr)));
        }

        if n == m {
            println!(
                "Memory state before step {} for the context {} at address {}:",
                self.clk, self.ctx, n
            )
        } else {
            println!(
                "Memory state before step {} for the context {} in the interval [{}, {}]:",
                self.clk, self.ctx, n, m
            )
        };

        print_interval(mem_interval);
    }

    /// Prints locals in provided interval.
    ///
    /// If `print_all` is true, the interval should contain all indexes of locals available for the
    /// current procedure.
    fn print_local_interval<S: ProcessState>(
        &self,
        process: &S,
        interval: (u32, u32),
        num_locals: u32,
        print_all: bool,
    ) {
        let mut local_mem_interval = Vec::new();
        let local_memory_offset = self.fmp - num_locals + 1;
        for index in interval.0..interval.1 + 1 {
            local_mem_interval
                .push((index, process.get_mem_value(self.ctx, index + local_memory_offset)))
        }

        if print_all {
            println!("Local state before step {} for the context {}:", self.clk, self.ctx)
        } else if interval.0 == interval.1 {
            println!(
                "Local state before step {} for the context {} at index {}:",
                self.clk, self.ctx, interval.0
            )
        } else {
            println!(
                "Local state before step {} for the context {} in the interval [{}, {}]:",
                self.clk, self.ctx, interval.0, interval.1
            )
        };

        print_interval(local_mem_interval);
    }
}

fn print_interval(mem_interval: Vec<(u32, Option<Word>)>) {
    // print the main part of the memory (wihtout the last value)
    for (addr, value) in mem_interval.iter().take(mem_interval.len() - 1) {
        if let Some(value) = value {
            print_word(*addr, value, false);
        } else {
            println!("├── {addr:.<10}: EMPTY");
        }
    }

    // print the last memory value
    if let Some((addr, value)) = mem_interval.last() {
        if let Some(value) = value {
            print_word(*addr, value, true);
        } else {
            println!("└── {addr:.<10}: EMPTY\n");
        }
    } else {
        println!("└── EMPTY\n");
    }
}

fn print_word(addr: u32, value: &Word, last: bool) {
    if last {
        print!("└── ");
    } else {
        print!("├── ");
    }
    println!("{addr:.<10}: [{}, {}, {}, {}]", value[0], value[1], value[2], value[3],);
    if last {
        println!();
    }
}

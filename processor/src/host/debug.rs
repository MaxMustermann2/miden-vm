use super::{Felt, ProcessState};
use crate::{Vec, Word};
use vm_core::{
    utils::string::{String, ToString},
    DebugOptions,
};

// DEBUG HANDLER
// ================================================================================================

/// Prints the info about the VM state specified by the provided options to stdout.
pub fn print_debug_info<S: ProcessState>(process: &S, options: &DebugOptions) {
    let clk = process.clk();
    let ctx = process.ctx();
    match options {
        DebugOptions::StackAll => {
            let stack = process.get_stack_state();
            let n = stack.len();
            print_vm_stack(clk, stack, n);
        }
        DebugOptions::StackTop(n) => {
            let stack = process.get_stack_state();
            print_vm_stack(clk, stack, *n as usize);
        }
        DebugOptions::MemAll => {
            let mem = process.get_mem(ctx);
            print_mem_all(clk, ctx, mem);
        }
        DebugOptions::MemAddr(n) => {
            let mem_value = process.get_mem_value(ctx, *n);
            print_mem(clk, ctx, *n, mem_value, false);
        }
        DebugOptions::MemInterval(n, m) => {
            let mut mem_interval = Vec::new();
            for addr in *n..*m + 1 {
                mem_interval.push((addr, process.get_mem_value(ctx, addr)));
            }
            let header = format!(
                "Memory state before step {clk} for the context {ctx} in the interval [{}, {}]:",
                *n, *m
            );
            print_mem_interval(mem_interval, header);
        }
        DebugOptions::LocalIndex(n) => {
            let local_mem_value = process.get_mem_value(ctx, 2u32.pow(30) + n + 1);
            print_mem(clk, ctx, *n, local_mem_value, true);
        }
        DebugOptions::LocalInterval(n, m, print_all) => {
            print_local_interval(process, clk, ctx, (*n, *m), *print_all)
        }
        DebugOptions::All(locals_num) => {
            // print stack
            let stack = process.get_stack_state();
            let n = stack.len();
            print_vm_stack(clk, stack, n);

            // print memory
            let mem = process.get_mem(ctx);
            print_mem_all(clk, ctx, mem);

            // print locals
            print_local_interval(process, clk, ctx, (0, *locals_num as u32 - 1), true);
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

#[cfg(feature = "std")]
fn print_vm_stack(clk: u32, stack: Vec<Felt>, n: usize) {
    // determine how many items to print out
    let num_items = core::cmp::min(stack.len(), n);

    // print all items except for the last one
    println!("Stack state before step {clk}:");
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
#[cfg(feature = "std")]
fn print_mem_all(clk: u32, ctx: u32, mem: Vec<(u64, Word)>) {
    println!("Memory state before step {clk} for the context {ctx}:");

    // calculate the proper padding for the memory addresses
    let padding = mem.iter().fold(0, |max, value| value.0.to_string().len().max(max));

    // print the main part of the memory (wihtout the last value)
    for (addr, value) in mem.iter().take(mem.len() - 1) {
        println!(
            "├── {addr:>width$}: [{}, {}, {}, {}]",
            value[0],
            value[1],
            value[2],
            value[3],
            width = padding
        );
    }

    // print the last memory value
    if let Some((addr, value)) = mem.last() {
        println!(
            "└── {addr:>width$}: [{}, {}, {}, {}]\n",
            value[0],
            value[1],
            value[2],
            value[3],
            width = padding
        );
    } else {
        println!("└── EMPTY\n");
    }
}

/// Prints the memory value stored at the requested address `addr`.
#[cfg(feature = "std")]
fn print_mem(clk: u32, ctx: u32, addr: u32, value: Option<Word>, local: bool) {
    if local {
        println!("Local state before step {} for the context {} at index {}:", clk, ctx, addr);
    } else {
        println!("Memory state before step {} for the context {} at address {}:", clk, ctx, addr);
    }

    if let Some(value) = value {
        println!("└── [{}, {}, {}, {}]\n", value[0], value[1], value[2], value[3],);
    } else {
        println!("└── EMPTY\n");
    }
}

/// Prints locals in provided interval.
///
/// If `print_all` is true, the interval should contain all indexes of locals available for the
/// current procedure.
#[cfg(feature = "std")]
fn print_local_interval<S: ProcessState>(
    process: &S,
    clk: u32,
    ctx: u32,
    interval: (u32, u32),
    print_all: bool,
) {
    let mut local_mem_interval = Vec::new();
    let local_memory_offset = 2u32.pow(30) + 1;
    for index in interval.0..interval.1 + 1 {
        local_mem_interval.push((index, process.get_mem_value(ctx, index + local_memory_offset)))
    }
    if print_all {
        print_mem_interval(
            local_mem_interval,
            format!("Local state before step {clk} for the context {ctx}:"),
        );
    } else {
        print_mem_interval(
            local_mem_interval,
            format!(
                "Local state before step {clk} for the context {ctx} in the interval [{}, {}]:",
                interval.0, interval.1
            ),
        );
    }
}

/// Prints memory values in the provided addresses interval.
///
/// `header` contains a title message corresponding to the DebugOption it is used in
/// ([DebugOptions::MemInterval] or [DebugOptions::LocalInterval]).
#[cfg(feature = "std")]
fn print_mem_interval(mem: Vec<(u32, Option<Word>)>, header: String) {
    println!("{header}");

    // calculate the proper padding for the memory addresses
    let padding = mem.iter().fold(0, |max, value| value.0.to_string().len().max(max));

    // print the main part of the memory (wihtout the last value)
    for (addr, value) in mem.iter().take(mem.len() - 1) {
        if let Some(value) = value {
            println!(
                "├── {addr:>width$}: [{}, {}, {}, {}]",
                value[0],
                value[1],
                value[2],
                value[3],
                width = padding
            );
        } else {
            println!("├── {addr:>width$}: EMPTY", width = padding);
        }
    }

    // print the last memory value
    if let Some((addr, value)) = mem.last() {
        if let Some(value) = value {
            println!(
                "└── {addr:>width$}: [{}, {}, {}, {}]\n",
                value[0],
                value[1],
                value[2],
                value[3],
                width = padding
            );
        } else {
            println!("└── {addr:>width$}: EMPTY\n", width = padding);
        }
    } else {
        // situation when `mem` vector is empty should be impossible -- we will either get parsing
        // error because of incorrect border indexes, or there is at least one element in the
        // provided `mem` vector
        unreachable!("Values provided to the debug.mem/debug.local operation are incorrect");
    }
}

#[cfg(not(feature = "std"))]
fn print_vm_stack(_clk: u32, _stack: Vec<Felt>, _n: usize) {
    // in no_std environments, this is a NOOP
}

#[cfg(not(feature = "std"))]
fn print_mem_all(_clk: u32, _ctx: u32, _mem: Vec<(u64, Word)>) {
    // in no_std environments, this is a NOOP
}

#[cfg(not(feature = "std"))]
fn print_mem(_clk: u32, _ctx: u32, _addr: u32, _value: Option<Word>, _local: bool) {
    // in no_std environments, this is a NOOP
}

#[cfg(not(feature = "std"))]
fn print_local_interval<S: ProcessState>(
    _process: &S,
    _clk: u32,
    _ctx: u32,
    _interval: (u32, u32),
    _print_all: bool,
) {
    // in no_std environments, this is a NOOP
}

#[cfg(not(feature = "std"))]
fn print_mem_interval(_mem: Vec<(u32, Option<Word>)>, _header: String) {
    // in no_std environments, this is a NOOP
}

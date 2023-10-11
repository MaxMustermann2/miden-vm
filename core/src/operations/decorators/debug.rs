use core::fmt;

// DEBUG OPTIONS
// ================================================================================================

/// Options of the `Debug` decorator.
///
/// These options define the debug info which gets printed out when the Debug decorator is
/// executed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DebugOptions {
    /// Print out the entire contents of the stack for the current execution context.
    StackAll,
    /// Prints out the top n items of the stack for the current context.
    StackTop(u16),
    /// Prints out the entire contents of RAM.
    MemAll,
    /// Prints out contents of memory at the provided address.
    MemAddr(u32),
    /// Prints out the contents of memory stored in the provided interval. Interval boundaries are
    /// both inclusive.
    MemInterval(u32, u32),
    /// Prints out contents of the local at the provided index for the currently executing
    /// procedure.
    LocalIndex(u32),
    /// Prints out locals stored in the provided interval of the currently executing procedure.
    /// Interval boundaries are both inclusive.
    LocalInterval(u32, u32, bool),
    /// Prints out the entire state of the VM (stack and RAM).
    All(u16),
}

impl fmt::Display for DebugOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackAll => write!(f, "stack"),
            Self::StackTop(n) => write!(f, "stack.{n}"),
            Self::MemAll => write!(f, "mem"),
            Self::MemAddr(n) => write!(f, "mem.{n}"),
            Self::MemInterval(n, m) => write!(f, "mem.{n}.{m}"),
            Self::LocalIndex(n) => write!(f, "local.{n}"),
            Self::LocalInterval(n, m, print_all) => {
                if *print_all {
                    write!(f, "local")
                } else {
                    write!(f, "local.{n}.{m}")
                }
            }
            Self::All(_) => write!(f, "all"),
        }
    }
}

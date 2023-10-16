use super::{super::DebugOptions, ByteReader, ByteWriter, DeserializationError, ToString};

const STACK_ALL: u8 = 0;
const STACK_TOP: u8 = 1;
const MEM_ALL: u8 = 2;
const MEM_INTERVAL: u8 = 3;
const LOCAL_INTERVAL: u8 = 4;
const ALL: u8 = 5;

/// Writes the provided [DebugOptions] into the provided target.
pub fn write_options_into<W: ByteWriter>(target: &mut W, options: &DebugOptions) {
    match options {
        DebugOptions::StackAll => target.write_u8(STACK_ALL),
        DebugOptions::StackTop(n) => {
            target.write_u8(STACK_TOP);
            target.write_u16(*n);
        }
        DebugOptions::MemAll => target.write_u8(MEM_ALL),
        DebugOptions::MemInterval(n, m) => {
            target.write_u8(MEM_INTERVAL);
            target.write_u32(*n);
            target.write_u32(*m);
        }
        DebugOptions::LocalInterval(interval, num_locals, print_all) => {
            target.write_u8(LOCAL_INTERVAL);
            target.write_u32(interval.0);
            target.write_u32(interval.1);
            target.write_u32(*num_locals);
            target.write_bool(*print_all);
        }
        DebugOptions::All(num_locals) => {
            target.write_u8(ALL);
            target.write_u32(*num_locals);
        }
    }
}

/// Reads [DebugOptions] from the provided source.
pub fn read_options_from<R: ByteReader>(
    source: &mut R,
) -> Result<DebugOptions, DeserializationError> {
    match source.read_u8()? {
        STACK_ALL => Ok(DebugOptions::StackAll),
        STACK_TOP => {
            let n = source.read_u16()?;
            if n == 0 {
                return Err(DeserializationError::InvalidValue(n.to_string()));
            }
            Ok(DebugOptions::StackTop(n))
        }
        MEM_ALL => Ok(DebugOptions::MemAll),
        MEM_INTERVAL => {
            let n = source.read_u32()?;
            let m = source.read_u32()?;
            Ok(DebugOptions::MemInterval(n, m))
        }
        LOCAL_INTERVAL => {
            let n = source.read_u32()?;
            let m = source.read_u32()?;
            let num_locals = source.read_u32()?;
            let print_all = source.read_bool()?;
            Ok(DebugOptions::LocalInterval((n, m), num_locals, print_all))
        }
        ALL => {
            let num_locals = source.read_u32()?;
            Ok(DebugOptions::All(num_locals))
        }
        val => Err(DeserializationError::InvalidValue(val.to_string())),
    }
}

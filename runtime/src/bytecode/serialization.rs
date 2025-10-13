/// PohLang Bytecode File Format (.pbc)
///
/// Serialization and deserialization for bytecode chunks
use super::{BytecodeChunk, Constant, DebugInfo, Instruction};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Magic header for .pbc files: "POHC" (PohLang Compiled)
const MAGIC: &[u8; 4] = b"POHC";

/// Current bytecode format version
const VERSION: u32 = 1;

/// Serialization errors
#[derive(Debug)]
pub enum SerializationError {
    IoError(io::Error),
    InvalidMagic,
    UnsupportedVersion(u32),
    InvalidData(String),
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::IoError(e) => write!(f, "I/O error: {}", e),
            SerializationError::InvalidMagic => write!(f, "Invalid magic header (not a .pbc file)"),
            SerializationError::UnsupportedVersion(v) => {
                write!(f, "Unsupported bytecode version: {}", v)
            }
            SerializationError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for SerializationError {}

impl From<io::Error> for SerializationError {
    fn from(err: io::Error) -> Self {
        SerializationError::IoError(err)
    }
}

pub type SerializationResult<T> = Result<T, SerializationError>;

/// Bytecode serializer
pub struct BytecodeSerializer;

impl BytecodeSerializer {
    /// Serialize a bytecode chunk to bytes
    pub fn serialize(chunk: &BytecodeChunk) -> SerializationResult<Vec<u8>> {
        let mut buf = Vec::new();

        // Write magic header
        buf.write_all(MAGIC)?;

        // Write version
        buf.write_all(&VERSION.to_le_bytes())?;

        // Write chunk version
        buf.write_all(&chunk.version.to_le_bytes())?;

        // Write constants
        Self::write_constants(&mut buf, &chunk.constants)?;

        // Write code
        Self::write_code(&mut buf, &chunk.code)?;

        // Write debug info (optional)
        Self::write_debug_info(&mut buf, &chunk.debug_info)?;

        Ok(buf)
    }

    /// Save a bytecode chunk to a file
    pub fn save_to_file<P: AsRef<Path>>(chunk: &BytecodeChunk, path: P) -> SerializationResult<()> {
        let bytes = Self::serialize(chunk)?;
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    /// Write constants section
    fn write_constants(buf: &mut Vec<u8>, constants: &[Constant]) -> io::Result<()> {
        // Write constant count
        buf.write_all(&(constants.len() as u32).to_le_bytes())?;

        for constant in constants {
            match constant {
                Constant::Number(n) => {
                    buf.push(0); // Type tag
                    buf.write_all(&n.to_le_bytes())?;
                }
                Constant::String(s) => {
                    buf.push(1); // Type tag
                    let bytes = s.as_bytes();
                    buf.write_all(&(bytes.len() as u32).to_le_bytes())?;
                    buf.write_all(bytes)?;
                }
                Constant::Boolean(b) => {
                    buf.push(2); // Type tag
                    buf.push(if *b { 1 } else { 0 });
                }
                Constant::Null => {
                    buf.push(3); // Type tag
                }
            }
        }

        Ok(())
    }

    /// Write code section
    fn write_code(buf: &mut Vec<u8>, code: &[Instruction]) -> io::Result<()> {
        // Write instruction count
        buf.write_all(&(code.len() as u32).to_le_bytes())?;

        for instruction in code {
            Self::write_instruction(buf, instruction)?;
        }

        Ok(())
    }

    /// Write a single instruction
    fn write_instruction(buf: &mut Vec<u8>, instruction: &Instruction) -> io::Result<()> {
        match instruction {
            Instruction::LoadConst(idx) => {
                buf.push(0);
                buf.write_all(&idx.to_le_bytes())?;
            }
            Instruction::LoadTrue => buf.push(1),
            Instruction::LoadFalse => buf.push(2),
            Instruction::LoadNull => buf.push(3),
            Instruction::LoadLocal(idx) => {
                buf.push(4);
                buf.write_all(&idx.to_le_bytes())?;
            }
            Instruction::StoreLocal(idx) => {
                buf.push(5);
                buf.write_all(&idx.to_le_bytes())?;
            }
            Instruction::LoadGlobal(name) => {
                buf.push(6);
                let bytes = name.as_bytes();
                buf.write_all(&(bytes.len() as u32).to_le_bytes())?;
                buf.write_all(bytes)?;
            }
            Instruction::StoreGlobal(name) => {
                buf.push(7);
                let bytes = name.as_bytes();
                buf.write_all(&(bytes.len() as u32).to_le_bytes())?;
                buf.write_all(bytes)?;
            }
            Instruction::Add => buf.push(10),
            Instruction::Subtract => buf.push(11),
            Instruction::Multiply => buf.push(12),
            Instruction::Divide => buf.push(13),
            Instruction::Negate => buf.push(14),
            Instruction::Equal => buf.push(20),
            Instruction::NotEqual => buf.push(21),
            Instruction::Greater => buf.push(22),
            Instruction::GreaterEqual => buf.push(23),
            Instruction::Less => buf.push(24),
            Instruction::LessEqual => buf.push(25),
            Instruction::Not => buf.push(26),
            Instruction::And => buf.push(27),
            Instruction::Or => buf.push(28),
            Instruction::Jump(offset) => {
                buf.push(30);
                buf.write_all(&offset.to_le_bytes())?;
            }
            Instruction::JumpIfFalse(offset) => {
                buf.push(31);
                buf.write_all(&offset.to_le_bytes())?;
            }
            Instruction::JumpIfTrue(offset) => {
                buf.push(32);
                buf.write_all(&offset.to_le_bytes())?;
            }
            Instruction::Loop(offset) => {
                buf.push(33);
                buf.write_all(&offset.to_le_bytes())?;
            }
            Instruction::Call(argc) => {
                buf.push(40);
                buf.push(*argc);
            }
            Instruction::Return => buf.push(41),
            Instruction::Pop => buf.push(42),
            Instruction::Print => buf.push(50),
            Instruction::Input => buf.push(51),
            Instruction::Halt => buf.push(99),
            _ => {
                // For unimplemented instructions, use opcode 255
                buf.push(255);
            }
        }
        Ok(())
    }

    /// Write debug info section
    fn write_debug_info(buf: &mut Vec<u8>, debug_info: &Option<DebugInfo>) -> io::Result<()> {
        match debug_info {
            Some(info) => {
                buf.push(1); // Has debug info

                // Write source file
                let bytes = info.source_file.as_bytes();
                buf.write_all(&(bytes.len() as u32).to_le_bytes())?;
                buf.write_all(bytes)?;

                // Write line numbers
                buf.write_all(&(info.line_numbers.len() as u32).to_le_bytes())?;
                for line in &info.line_numbers {
                    buf.write_all(&line.to_le_bytes())?;
                }

                // Write variable names
                buf.write_all(&(info.variable_names.len() as u32).to_le_bytes())?;
                for name in &info.variable_names {
                    let bytes = name.as_bytes();
                    buf.write_all(&(bytes.len() as u32).to_le_bytes())?;
                    buf.write_all(bytes)?;
                }
            }
            None => {
                buf.push(0); // No debug info
            }
        }
        Ok(())
    }
}

/// Bytecode deserializer
pub struct BytecodeDeserializer;

impl BytecodeDeserializer {
    /// Deserialize bytes to a bytecode chunk
    pub fn deserialize(bytes: &[u8]) -> SerializationResult<BytecodeChunk> {
        let mut cursor = 0;

        // Read magic header
        if bytes.len() < 4 || &bytes[0..4] != MAGIC {
            return Err(SerializationError::InvalidMagic);
        }
        cursor += 4;

        // Read version
        let version = Self::read_u32(bytes, &mut cursor)?;
        if version != VERSION {
            return Err(SerializationError::UnsupportedVersion(version));
        }

        // Read chunk version
        let chunk_version = Self::read_u32(bytes, &mut cursor)?;

        // Read constants
        let constants = Self::read_constants(bytes, &mut cursor)?;

        // Read code
        let code = Self::read_code(bytes, &mut cursor)?;

        // Read debug info
        let debug_info = Self::read_debug_info(bytes, &mut cursor)?;

        Ok(BytecodeChunk {
            version: chunk_version,
            constants,
            code,
            debug_info,
        })
    }

    /// Load a bytecode chunk from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> SerializationResult<BytecodeChunk> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Self::deserialize(&bytes)
    }

    /// Read u32 from bytes
    fn read_u32(bytes: &[u8], cursor: &mut usize) -> SerializationResult<u32> {
        if *cursor + 4 > bytes.len() {
            return Err(SerializationError::InvalidData(
                "Unexpected end of data".to_string(),
            ));
        }
        let value = u32::from_le_bytes([
            bytes[*cursor],
            bytes[*cursor + 1],
            bytes[*cursor + 2],
            bytes[*cursor + 3],
        ]);
        *cursor += 4;
        Ok(value)
    }

    /// Read i32 from bytes
    fn read_i32(bytes: &[u8], cursor: &mut usize) -> SerializationResult<i32> {
        if *cursor + 4 > bytes.len() {
            return Err(SerializationError::InvalidData(
                "Unexpected end of data".to_string(),
            ));
        }
        let value = i32::from_le_bytes([
            bytes[*cursor],
            bytes[*cursor + 1],
            bytes[*cursor + 2],
            bytes[*cursor + 3],
        ]);
        *cursor += 4;
        Ok(value)
    }

    /// Read f64 from bytes
    fn read_f64(bytes: &[u8], cursor: &mut usize) -> SerializationResult<f64> {
        if *cursor + 8 > bytes.len() {
            return Err(SerializationError::InvalidData(
                "Unexpected end of data".to_string(),
            ));
        }
        let value = f64::from_le_bytes([
            bytes[*cursor],
            bytes[*cursor + 1],
            bytes[*cursor + 2],
            bytes[*cursor + 3],
            bytes[*cursor + 4],
            bytes[*cursor + 5],
            bytes[*cursor + 6],
            bytes[*cursor + 7],
        ]);
        *cursor += 8;
        Ok(value)
    }

    /// Read string from bytes
    fn read_string(bytes: &[u8], cursor: &mut usize) -> SerializationResult<String> {
        let len = Self::read_u32(bytes, cursor)? as usize;
        if *cursor + len > bytes.len() {
            return Err(SerializationError::InvalidData(
                "String length exceeds data".to_string(),
            ));
        }
        let s = String::from_utf8(bytes[*cursor..*cursor + len].to_vec())
            .map_err(|e| SerializationError::InvalidData(format!("Invalid UTF-8: {}", e)))?;
        *cursor += len;
        Ok(s)
    }

    /// Read constants section
    fn read_constants(bytes: &[u8], cursor: &mut usize) -> SerializationResult<Vec<Constant>> {
        let count = Self::read_u32(bytes, cursor)? as usize;
        let mut constants = Vec::with_capacity(count);

        for _ in 0..count {
            if *cursor >= bytes.len() {
                return Err(SerializationError::InvalidData(
                    "Unexpected end in constants".to_string(),
                ));
            }
            let type_tag = bytes[*cursor];
            *cursor += 1;

            let constant = match type_tag {
                0 => {
                    let n = Self::read_f64(bytes, cursor)?;
                    Constant::Number(n)
                }
                1 => {
                    let s = Self::read_string(bytes, cursor)?;
                    Constant::String(s)
                }
                2 => {
                    if *cursor >= bytes.len() {
                        return Err(SerializationError::InvalidData(
                            "Boolean missing".to_string(),
                        ));
                    }
                    let b = bytes[*cursor] != 0;
                    *cursor += 1;
                    Constant::Boolean(b)
                }
                3 => Constant::Null,
                _ => {
                    return Err(SerializationError::InvalidData(format!(
                        "Unknown constant type: {}",
                        type_tag
                    )))
                }
            };

            constants.push(constant);
        }

        Ok(constants)
    }

    /// Read code section
    fn read_code(bytes: &[u8], cursor: &mut usize) -> SerializationResult<Vec<Instruction>> {
        let count = Self::read_u32(bytes, cursor)? as usize;
        let mut code = Vec::with_capacity(count);

        for _ in 0..count {
            let instruction = Self::read_instruction(bytes, cursor)?;
            code.push(instruction);
        }

        Ok(code)
    }

    /// Read a single instruction
    fn read_instruction(bytes: &[u8], cursor: &mut usize) -> SerializationResult<Instruction> {
        if *cursor >= bytes.len() {
            return Err(SerializationError::InvalidData(
                "Unexpected end in code".to_string(),
            ));
        }
        let opcode = bytes[*cursor];
        *cursor += 1;

        let instruction = match opcode {
            0 => Instruction::LoadConst(Self::read_u32(bytes, cursor)?),
            1 => Instruction::LoadTrue,
            2 => Instruction::LoadFalse,
            3 => Instruction::LoadNull,
            4 => Instruction::LoadLocal(Self::read_u32(bytes, cursor)?),
            5 => Instruction::StoreLocal(Self::read_u32(bytes, cursor)?),
            6 => Instruction::LoadGlobal(Self::read_string(bytes, cursor)?),
            7 => Instruction::StoreGlobal(Self::read_string(bytes, cursor)?),
            10 => Instruction::Add,
            11 => Instruction::Subtract,
            12 => Instruction::Multiply,
            13 => Instruction::Divide,
            14 => Instruction::Negate,
            20 => Instruction::Equal,
            21 => Instruction::NotEqual,
            22 => Instruction::Greater,
            23 => Instruction::GreaterEqual,
            24 => Instruction::Less,
            25 => Instruction::LessEqual,
            26 => Instruction::Not,
            27 => Instruction::And,
            28 => Instruction::Or,
            30 => Instruction::Jump(Self::read_i32(bytes, cursor)?),
            31 => Instruction::JumpIfFalse(Self::read_i32(bytes, cursor)?),
            32 => Instruction::JumpIfTrue(Self::read_i32(bytes, cursor)?),
            33 => Instruction::Loop(Self::read_i32(bytes, cursor)?),
            40 => {
                if *cursor >= bytes.len() {
                    return Err(SerializationError::InvalidData(
                        "Call argc missing".to_string(),
                    ));
                }
                let argc = bytes[*cursor];
                *cursor += 1;
                Instruction::Call(argc)
            }
            41 => Instruction::Return,
            42 => Instruction::Pop,
            50 => Instruction::Print,
            51 => Instruction::Input,
            99 => Instruction::Halt,
            255 => Instruction::Halt, // Fallback for unsupported instructions
            _ => {
                return Err(SerializationError::InvalidData(format!(
                    "Unknown opcode: {}",
                    opcode
                )))
            }
        };

        Ok(instruction)
    }

    /// Read debug info section
    fn read_debug_info(bytes: &[u8], cursor: &mut usize) -> SerializationResult<Option<DebugInfo>> {
        if *cursor >= bytes.len() {
            return Ok(None);
        }

        let has_debug = bytes[*cursor] != 0;
        *cursor += 1;

        if !has_debug {
            return Ok(None);
        }

        // Read source file
        let source_file = Self::read_string(bytes, cursor)?;

        // Read line numbers
        let line_count = Self::read_u32(bytes, cursor)? as usize;
        let mut line_numbers = Vec::with_capacity(line_count);
        for _ in 0..line_count {
            line_numbers.push(Self::read_u32(bytes, cursor)?);
        }

        // Read variable names
        let var_count = Self::read_u32(bytes, cursor)? as usize;
        let mut variable_names = Vec::with_capacity(var_count);
        for _ in 0..var_count {
            variable_names.push(Self::read_string(bytes, cursor)?);
        }

        Ok(Some(DebugInfo {
            source_file,
            line_numbers,
            variable_names,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_simple() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(42.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::Return);

        let bytes = BytecodeSerializer::serialize(&chunk).unwrap();
        let deserialized = BytecodeDeserializer::deserialize(&bytes).unwrap();

        assert_eq!(deserialized.version, 1);
        assert_eq!(deserialized.constants.len(), 1);
        assert_eq!(deserialized.code.len(), 2);
    }

    #[test]
    fn test_all_constant_types() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(3.14));
        chunk.constants.push(Constant::String("hello".to_string()));
        chunk.constants.push(Constant::Boolean(true));
        chunk.constants.push(Constant::Null);
        chunk.code.push(Instruction::Halt);

        let bytes = BytecodeSerializer::serialize(&chunk).unwrap();
        let deserialized = BytecodeDeserializer::deserialize(&bytes).unwrap();

        assert_eq!(deserialized.constants.len(), 4);
        assert!(matches!(deserialized.constants[0], Constant::Number(_)));
        assert!(matches!(deserialized.constants[1], Constant::String(_)));
        assert!(matches!(deserialized.constants[2], Constant::Boolean(true)));
        assert!(matches!(deserialized.constants[3], Constant::Null));
    }

    #[test]
    fn test_invalid_magic() {
        let bytes = vec![1, 2, 3, 4]; // Invalid magic
        let result = BytecodeDeserializer::deserialize(&bytes);
        assert!(matches!(result, Err(SerializationError::InvalidMagic)));
    }
}

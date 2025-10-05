// Minimal bytecode instruction set and (very) simple encoding.

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PushNum(f64),
    PushStr(String),
    PushBool(bool),
    PushNull,
    LoadVar(String),
    StoreVar(String),
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Jump(usize),
    JumpIfFalse(usize),
    WriteTop,
    AskVar(String),
}

impl Instruction {
    pub fn encode(&self) -> String {
        match self {
            Instruction::PushNum(n) => format!("PUSH_NUM\t{}", n),
            Instruction::PushStr(s) => format!("PUSH_STR\t{}", hex_encode(s.as_bytes())),
            Instruction::PushBool(b) => format!("PUSH_BOOL\t{}", if *b { 1 } else { 0 }),
            Instruction::PushNull => "PUSH_NULL".to_string(),
            Instruction::LoadVar(n) => format!("LOAD_VAR\t{}", n),
            Instruction::StoreVar(n) => format!("STORE_VAR\t{}", n),
            Instruction::Add => "ADD".to_string(),
            Instruction::Sub => "SUB".to_string(),
            Instruction::Mul => "MUL".to_string(),
            Instruction::Div => "DIV".to_string(),
            Instruction::Eq => "EQ".to_string(),
            Instruction::Ne => "NE".to_string(),
            Instruction::Lt => "LT".to_string(),
            Instruction::Le => "LE".to_string(),
            Instruction::Gt => "GT".to_string(),
            Instruction::Ge => "GE".to_string(),
            Instruction::Jump(t) => format!("JMP\t{}", t),
            Instruction::JumpIfFalse(t) => format!("JMPF\t{}", t),
            Instruction::WriteTop => "WRITE_TOP".to_string(),
            Instruction::AskVar(n) => format!("ASK_VAR\t{}", n),
        }
    }

    pub fn decode(line: &str) -> Option<Instruction> {
        let mut parts = line.splitn(2, '\t');
        let op = parts.next()?;
        let arg = parts.next().unwrap_or("");
        match op {
            "PUSH_NUM" => arg.parse::<f64>().ok().map(Instruction::PushNum),
            "PUSH_STR" => hex_decode(arg)
                .ok()
                .and_then(|b| String::from_utf8(b).ok())
                .map(Instruction::PushStr),
            "PUSH_BOOL" => arg
                .parse::<i32>()
                .ok()
                .map(|i| Instruction::PushBool(i != 0)),
            "PUSH_NULL" => Some(Instruction::PushNull),
            "LOAD_VAR" => Some(Instruction::LoadVar(arg.to_string())),
            "STORE_VAR" => Some(Instruction::StoreVar(arg.to_string())),
            "ADD" => Some(Instruction::Add),
            "SUB" => Some(Instruction::Sub),
            "MUL" => Some(Instruction::Mul),
            "DIV" => Some(Instruction::Div),
            "EQ" => Some(Instruction::Eq),
            "NE" => Some(Instruction::Ne),
            "LT" => Some(Instruction::Lt),
            "LE" => Some(Instruction::Le),
            "GT" => Some(Instruction::Gt),
            "GE" => Some(Instruction::Ge),
            "JMP" => arg.parse::<usize>().ok().map(Instruction::Jump),
            "JMPF" => arg.parse::<usize>().ok().map(Instruction::JumpIfFalse),
            "WRITE_TOP" => Some(Instruction::WriteTop),
            "ASK_VAR" => Some(Instruction::AskVar(arg.to_string())),
            _ => None,
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ()> {
    let mut out = Vec::with_capacity(s.len() / 2);
    let mut chars = s.as_bytes().chunks_exact(2);
    for ch in &mut chars {
        let hi = from_hex(ch[0])?;
        let lo = from_hex(ch[1])?;
        out.push((hi << 4) | lo);
    }
    if !chars.remainder().is_empty() {
        return Err(());
    }
    Ok(out)
}

fn from_hex(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}

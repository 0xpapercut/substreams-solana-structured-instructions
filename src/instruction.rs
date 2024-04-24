use substreams_solana::pb::sf::solana::r#type::v1 as pb;

#[derive(Debug)]
pub(crate) enum WrappedInstruction<'a> {
    Compiled(&'a pb::CompiledInstruction),
    Inner(&'a pb::InnerInstruction),
}

impl WrappedInstruction<'_> {
    pub fn program_id_index(&self) -> u32 { 
        match self {
            Self::Compiled(instr) => instr.program_id_index,
            Self::Inner(instr) => instr.program_id_index,
        }
    }
    pub fn accounts(&self) -> &Vec<u8> {
        match self {
            Self::Compiled(instr) => &instr.accounts,
            Self::Inner(instr) => &instr.accounts,
        }
    }
    pub fn data(&self) -> &Vec<u8> {
        match self {
            Self::Compiled(instr) => &instr.data,
            Self::Inner(instr) => &instr.data,
        }
    }
    pub fn stack_height(&self) -> Option<u32> {
        match self {
            Self::Compiled(_) => Some(1),
            Self::Inner(instr) => instr.stack_height,
        }
    }
}

impl<'a> From<&'a pb::CompiledInstruction> for WrappedInstruction<'a> {
    fn from(value: &'a pb::CompiledInstruction) -> Self {
        WrappedInstruction::Compiled(&value)
    }
}

impl<'a> From<&'a pb::InnerInstruction> for WrappedInstruction<'a> {
    fn from(value: &'a pb::InnerInstruction) -> Self {
        WrappedInstruction::Inner(&value)
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub program_id_index: u32,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    pub stack_height: u32,
    pub inner_instructions: Vec<Self>,
    pub logs: Vec<String>,
}

impl Instruction {
    fn new(instruction: &WrappedInstruction, inner_instructions: Vec<Instruction>) -> Self {
        Self {
            program_id_index: instruction.program_id_index(),
            accounts: instruction.accounts().clone(),
            data: instruction.data().clone(),
            stack_height: instruction.stack_height().unwrap(),
            inner_instructions: inner_instructions,
            logs: Vec::new(),
        }
    }
}

pub(crate) fn structure_wrapped_instructions_with_logs<'a>(instructions: &'a [WrappedInstruction], logs: &[String]) -> Vec<Instruction> {
    let mut structured_instructions: Vec<Instruction> = Vec::new();
    
    if instructions.len() == 0 {
        return Vec::new();
    }

    let stack_height = instructions[0].stack_height();
    let mut i = 0;
    for (j, instr) in instructions.iter().skip(1).enumerate() {
        if instr.stack_height() == stack_height && i != j {
            if j > i + 1 {
                let inner_instructions = structure_wrapped_instructions_with_logs(&instructions[i + 1..j - 1], logs);
                structured_instructions.push(Instruction::new(instr, inner_instructions))
            } else {
                structured_instructions.push(Instruction::new(instr, Vec::new()))
            }
            i = j;
        }
    }
    let inner_instructions = structure_wrapped_instructions_with_logs(&instructions[i + 1..], logs);
    structured_instructions.push(Instruction::new(&instructions[i], inner_instructions));

    structured_instructions
}

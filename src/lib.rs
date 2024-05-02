use substreams_solana::pb::sf::solana::r#type::v1 as pb;

mod instruction;
pub use instruction::*;

pub trait StructuredInstructions {
    fn flattened(&self) -> Vec<&StructuredInstruction>;
}

impl StructuredInstructions for Vec<StructuredInstruction> {
    fn flattened(&self) -> Vec<&StructuredInstruction> {
        let mut instructions: Vec<&StructuredInstruction> = Vec::new();
        for instruction in self {
            instructions.push(instruction);
            instructions.extend(instruction.inner_instructions.flattened());
        }
        instructions
    }
}

fn get_wrapped_instructions(confirmed_transaction: &pb::ConfirmedTransaction) -> Vec<WrappedInstruction> {
    let compiled_instructions = confirmed_transaction.transaction.as_ref().map(|x| x.message.as_ref().map(|y| &y.instructions)).unwrap().unwrap();
    let inner_instructions = confirmed_transaction.meta.as_ref().map(|x| &x.inner_instructions).unwrap();

    let mut wrapped_instructions: Vec<WrappedInstruction> = Vec::new();
    let mut j = 0;
    for (i, instr) in compiled_instructions.iter().enumerate() {
        wrapped_instructions.push(instr.into());
        if let Some(inner) = inner_instructions.get(j) {
            if inner.index == i as u32 {
                wrapped_instructions.extend(inner_instructions[j].instructions.iter().map(|x| x.into()));
                j += 1;
            }
        }
    }
    wrapped_instructions
}

pub fn get_structured_instructions(transaction: &pb::ConfirmedTransaction) -> Vec<StructuredInstruction> {
    let wrapped_instructions = get_wrapped_instructions(transaction);
    structure_wrapped_instructions_with_logs(&wrapped_instructions, &Vec::new())
}

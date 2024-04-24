use substreams_solana::pb::sf::solana::r#type::v1 as pb;
use crate::instruction::{structure_wrapped_instructions_with_logs, Instruction, WrappedInstruction};

#[derive(Debug)]
pub struct ConfirmedTransaction {
    pub transaction: Option<Transaction>,
    pub meta: Option<pb::TransactionStatusMeta>,
}

impl<'a> From<&'a pb::ConfirmedTransaction> for ConfirmedTransaction {
    fn from(value: &pb::ConfirmedTransaction) -> Self {
        let wrapped_instructions = get_wrapped_instructions(&value);
        let structured_instructions = structure_wrapped_instructions_with_logs(&wrapped_instructions, &Vec::new());

        let _transaction = value.transaction.as_ref().unwrap();
        let _signatures = &_transaction.signatures;
        let _message = _transaction.message.as_ref().unwrap();

        let message = Message {
            header: _message.header.clone(),
            account_keys: _message.account_keys.clone(),
            recent_blockhash: _message.recent_blockhash.clone(),
            instructions: structured_instructions,
            versioned: _message.versioned,
            address_table_lookups: _message.address_table_lookups.clone(),
        };
        let transaction = Transaction {
            signatures: _signatures.clone(),
            message: Some(message),
        };
        
        Self {
            transaction: Some(transaction),
            meta: value.meta.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub signatures: Vec<Vec<u8>>,
    pub message: Option<Message>,
}

#[derive(Debug)]
pub struct Message {
    pub header: Option<pb::MessageHeader>,
    pub account_keys: Vec<Vec<u8>>,
    pub recent_blockhash: Vec<u8>,
    pub instructions: Vec<Instruction>,
    pub versioned: bool,
    pub address_table_lookups: Vec<pb::MessageAddressTableLookup>,
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

pub fn get_structured_instructions(transaction: &pb::ConfirmedTransaction) -> Vec<Instruction> {
    let wrapped_instructions = get_wrapped_instructions(transaction);
    structure_wrapped_instructions_with_logs(&wrapped_instructions, &Vec::new())
}

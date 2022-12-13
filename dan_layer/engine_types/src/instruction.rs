//  Copyright 2022 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use tari_bor::{borsh, Encode};
use tari_crypto::ristretto::{RistrettoComSig, RistrettoPublicKey};
use tari_mmr::MerkleProof;
use tari_template_lib::{
    args::{Arg, LogLevel},
    models::{ComponentAddress, TemplateAddress},
    Hash,
};

use crate::hashing::hasher;

#[derive(Debug, Clone, Encode, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Instruction {
    CallFunction {
        template_address: TemplateAddress,
        function: String,
        args: Vec<Arg>,
    },
    CallMethod {
        component_address: ComponentAddress,
        method: String,
        args: Vec<Arg>,
    },
    PutLastInstructionOutputOnWorkspace {
        key: Vec<u8>,
    },
    EmitLog {
        level: LogLevel,
        message: String,
    },
    ImportUtxo {
        commitment: RistrettoPublicKey,
        proof_of_knowledge: RistrettoComSig,
        proof_of_existence: MerkleProof,
    },
}

impl Instruction {
    pub fn hash(&self) -> Hash {
        hasher("instruction").chain(self).result()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CallFunction {
                template_address,
                function,
                args,
            } => write!(
                f,
                "CallFunction {{ template_address: {}, function: {}, args: {:?} }}",
                template_address, function, args
            ),
            Self::CallMethod {
                component_address,
                method,
                args,
            } => write!(
                f,
                "CallMethod {{ component_address: {}, method: {}, args: {:?} }}",
                component_address, method, args
            ),
            Self::PutLastInstructionOutputOnWorkspace { key } => {
                write!(f, "PutLastInstructionOutputOnWorkspace {{ key: {:?} }}", key)
            },
            Self::EmitLog { level, message } => {
                write!(f, "EmitLog {{ level: {:?}, message: {:?} }}", level, message)
            },
            Self::ImportUtxo {
                commitment,
                proof_of_knowledge,
                proof_of_existence,
            } => {
                write!(
                    f,
                    "ImportUtxo {{ commitment: {:?}, proof_of_knowledge: {:?}, proof_of_existence: {:?} }}",
                    commitment, proof_of_knowledge, proof_of_existence
                )
            },
        }
    }
}

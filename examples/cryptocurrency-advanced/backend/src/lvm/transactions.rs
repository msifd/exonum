use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionSet, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::proto;
use super::{schema::Schema, service::LVM_SERVICE_ID};

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateContract")]
pub struct CreateContract {
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CallContract")]
pub struct CallContract {
    pub contract: PublicKey,
    pub exec_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum LvmTransactions {
    CreateContract(CreateContract),
}

impl CreateContract {
    #[doc(hidden)]
    pub fn sign(code: &str, pk: &PublicKey, sk: &SecretKey) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                code: code.to_owned(),
            },
            LVM_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for CreateContract {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        Ok(())
    }
}

impl Transaction for CallContract {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        Ok(())
    }
}

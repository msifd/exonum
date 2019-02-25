use exonum::{
    blockchain::{ExecutionError, ExecutionResult, Transaction, TransactionSet, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::proto;
use super::{schema::Schema, service::LVM_SERVICE_ID};

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Contract already exists")]
    ContractAlreadyExists = 0,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateContract")]
pub struct CreateContract {
    pub pub_key: PublicKey,
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
    pub fn sign(pub_key: &PublicKey, code: &str, pk: &PublicKey, sk: &SecretKey) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                pub_key: *pub_key,
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
        let mut schema = Schema::new(context.fork());

        if schema.contract(&self.pub_key).is_none() {
            schema.create_contract(&self.pub_key, &self.code);
            Ok(())
        } else {
            Err(Error::ContractAlreadyExists)?
        }
    }
}

impl Transaction for CallContract {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        Ok(())
    }
}

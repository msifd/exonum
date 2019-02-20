use exonum::crypto::{Hash, PublicKey};
use std::collections::HashMap;

use super::proto;

#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Contract", serde_pb_convert)]
pub struct Contract {
    pub pub_key: PublicKey,
    pub code: String,
    pub state: HashMap<String, String>,
}

use exonum::{
    crypto::{Hash, PublicKey},
    storage::Fork,
};

use std::{
    collections::HashMap,
    iter::FromIterator,
};

use super::{
    proto,
    schema::Schema,
};

#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Contract", serde_pb_convert)]
pub struct Contract {
    pub pub_key: PublicKey,
    pub code: String,
    pub state: HashMap<String, String>,
}

impl Contract {
    pub fn new(pub_key: &PublicKey, code: &str) -> Self {
        Self {
            pub_key: *pub_key,
            code: code.to_string(),
            state: HashMap::new(),
        }
    }

    pub fn exec(self, schema: &mut Schema<&mut Fork>, fn_name: &str, args: &Vec<String>) -> Result<Contract, rlua::Error> {
        use rlua::{Lua, Variadic, Function};
        
        let contract = self.clone();
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            lua_ctx.load(&contract.code);

            let func: Function = globals.get(fn_name)?;
            func.call::<_, ()>(Variadic::from_iter(args.iter().cloned()))?;

            Ok(contract)
        })
    }
}
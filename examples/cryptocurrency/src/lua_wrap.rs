extern crate hex;

use rlua::{ToLua, FromLua, Context, Value};
// use rlua::{UserData, UserDataMethods, MetaMethod};
use exonum::crypto::PublicKey;
use hex::FromHex;

pub struct LuaPublicKey(pub PublicKey);

// impl UserData for LuaPublicKey {
    // fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    //     methods.add_meta_method(MetaMethod::ToString, |_, this, val| {
    //         Ok(this.0.to_hex())
    //     });
    // }
// }

// impl<'lua> ToLua<'lua> for LuaPublicKey {
//     fn to_lua(self, lua: Context<'lua>) -> rlua::Result<Value<'lua>> {
//         Ok(Value::String(lua.create_string(&self.0.to_hex())?))
//     }
// }

impl<'lua> FromLua<'lua> for LuaPublicKey {
    fn from_lua(value: Value<'lua>, lua: Context<'lua>) -> rlua::Result<Self> {
        let hex = lua.coerce_string(value)?.unwrap().to_str().to_owned()?;
        let bytes = FromHex::from_hex(&hex)?;
        Ok(Self(PublicKey::from_slice(&bytes)?))
    }
}

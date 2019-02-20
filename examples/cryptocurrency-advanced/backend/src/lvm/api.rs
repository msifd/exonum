use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
};

#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope();
    }
}

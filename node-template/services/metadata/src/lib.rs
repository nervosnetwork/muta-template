mod types;

use derive_more::{Display, From};

use binding_macro::{cycles, read, service, write};
use protocol::fixed_codec::FixedCodec;
use protocol::traits::{ReturnEmpty, Service, ServiceSDK, StoreMap, RETURN_EMPTY};
use protocol::types::{Hash, Metadata, ServiceContext, METADATA_KEY};
use protocol::{ProtocolError, ProtocolErrorKind, ProtocolResult};

use crate::types::EmptyPayload;

pub struct MetadataService<SDK: ServiceSDK> {
    sdk: SDK,
}

#[service]
impl<SDK: 'static + ServiceSDK> MetadataService<SDK> {
    pub fn init(sdk: SDK) -> ProtocolResult<Self> {
        Ok(Self { sdk })
    }

    #[cycles(210_00)]
    #[read]
    fn get_metadata(&self, ctx: ServiceContext, _: EmptyPayload) -> ProtocolResult<Metadata> {
        let metadata: Metadata = self
            .sdk
            .get_value(&METADATA_KEY.to_owned())?
            .expect("Metadata should always be in the genesis block");
        Ok(metadata)
    }
}

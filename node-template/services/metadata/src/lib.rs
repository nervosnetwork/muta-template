mod types;

use derive_more::{Display, From};

use binding_macro::{cycles, init, read, service, write};
use protocol::fixed_codec::FixedCodec;
use protocol::traits::{RequestContext, ReturnEmpty, ServiceSDK, StoreMap, RETURN_EMPTY};
use protocol::types::{Hash, Metadata, METADATA_KEY};
use protocol::{ProtocolError, ProtocolErrorKind, ProtocolResult};

use crate::types::EmptyPayload;

pub struct MetadataService<SDK: ServiceSDK> {
    sdk: SDK,
}

#[service]
impl<SDK: ServiceSDK> MetadataService<SDK> {
    #[init]
    fn init(sdk: SDK) -> ProtocolResult<Self> {
        Ok(Self { sdk })
    }

    #[read]
    fn get_metadata<Context: RequestContext>(
        &self,
        ctx: Context,
        _: EmptyPayload,
    ) -> ProtocolResult<Metadata> {
        let metadata: Metadata = self
            .sdk
            .get_value(&METADATA_KEY.to_owned())?
            .expect("Metadata should always be in the genesis block");
        Ok(metadata)
    }
}

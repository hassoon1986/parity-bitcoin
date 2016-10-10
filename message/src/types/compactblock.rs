use ser::{Stream, Reader};
use common::BlockHeaderAndIDs;
use {PayloadType, MessageResult};

#[derive(Debug, PartialEq)]
pub struct CompactBlock {
	header: BlockHeaderAndIDs,
}

impl PayloadType for CompactBlock {
	fn version() -> u32 {
		70014
	}

	fn command() -> &'static str {
		"cmpctblock"
	}

	fn deserialize_payload(reader: &mut Reader, _version: u32) -> MessageResult<Self> where Self: Sized {
		let block = CompactBlock {
			header: try!(reader.read()),
		};

		Ok(block)
	}

	fn serialize_payload(&self, stream: &mut Stream, _version: u32) -> MessageResult<()> {
		stream.append(&self.header);
		Ok(())
	}
}
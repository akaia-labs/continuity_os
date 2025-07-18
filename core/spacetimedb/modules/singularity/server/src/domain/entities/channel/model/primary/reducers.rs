use spacetimedb::{ReducerContext, Table, reducer};

use super::{super::ChannelMetadata, PrimaryChannel, primary_channel};
use crate::common::types::StUuid;

#[reducer]
/// Creates a new primary channel.
pub fn create_superchannel(
	ctx: &ReducerContext, alias: String, metadata: Option<ChannelMetadata>,
) -> Result<(), String> {
	ctx.db
		.primary_channel()
		.try_insert(PrimaryChannel {
			id:              StUuid::new(ctx).to_string(),
			canonical_alias: alias,
			creator:         ctx.sender,
			created_at:      ctx.timestamp,
			updated_at:      ctx.timestamp,
			metadata:        metadata.unwrap_or_default(),
			subchannels:     vec![],
		})
		.map(|_| ())
		.map_err(|e| e.to_string())
}

#[reducer]
/// Creates a record for an existing channel space
/// bridged from an external source.
pub fn register_superchannel(
	ctx: &ReducerContext, channel_id: String, alias: String, metadata: Option<ChannelMetadata>,
) -> Result<(), String> {
	ctx.db
		.primary_channel()
		.try_insert(PrimaryChannel {
			id:              channel_id,
			canonical_alias: alias,
			creator:         ctx.sender,
			created_at:      ctx.timestamp,
			updated_at:      ctx.timestamp,
			metadata:        metadata.unwrap_or_default(),
			subchannels:     vec![],
		})
		.map(|_| ())
		.map_err(|e| e.to_string())
}

// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::channel_metadata_type::ChannelMetadata;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct CreateStandaloneChannelArgs {
	pub alias:    String,
	pub metadata: Option<ChannelMetadata>,
}

impl From<CreateStandaloneChannelArgs> for super::Reducer {
	fn from(args: CreateStandaloneChannelArgs) -> Self {
		Self::CreateStandaloneChannel {
			alias:    args.alias,
			metadata: args.metadata,
		}
	}
}

impl __sdk::InModule for CreateStandaloneChannelArgs {
	type Module = super::RemoteModule;
}

pub struct CreateStandaloneChannelCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `create_standalone_channel`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait create_standalone_channel {
	/// Request that the remote module invoke the reducer
	/// `create_standalone_channel` to run as soon as possible.
	///
	/// This method returns immediately, and errors only if we are unable to
	/// send the request. The reducer will run asynchronously in the future,
	///  and its status can be observed by listening for
	/// [`Self::on_create_standalone_channel`] callbacks.
	fn create_standalone_channel(
		&self, alias: String, metadata: Option<ChannelMetadata>,
	) -> __sdk::Result<()>;
	/// Register a callback to run whenever we are notified of an invocation of
	/// the reducer `create_standalone_channel`.
	///
	/// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the
	/// [`super::ReducerEventContext`] to determine the reducer's status.
	///
	/// The returned [`CreateStandaloneChannelCallbackId`] can be passed to
	/// [`Self::remove_on_create_standalone_channel`] to cancel the callback.
	fn on_create_standalone_channel(
		&self,
		callback: impl FnMut(&super::ReducerEventContext, &String, &Option<ChannelMetadata>)
		+ Send
		+ 'static,
	) -> CreateStandaloneChannelCallbackId;
	/// Cancel a callback previously registered by
	/// [`Self::on_create_standalone_channel`], causing it not to run in the
	/// future.
	fn remove_on_create_standalone_channel(&self, callback: CreateStandaloneChannelCallbackId);
}

impl create_standalone_channel for super::RemoteReducers {
	fn create_standalone_channel(
		&self, alias: String, metadata: Option<ChannelMetadata>,
	) -> __sdk::Result<()> {
		self.imp
			.call_reducer("create_standalone_channel", CreateStandaloneChannelArgs {
				alias,
				metadata,
			})
	}

	fn on_create_standalone_channel(
		&self,
		mut callback: impl FnMut(&super::ReducerEventContext, &String, &Option<ChannelMetadata>)
		+ Send
		+ 'static,
	) -> CreateStandaloneChannelCallbackId {
		CreateStandaloneChannelCallbackId(self.imp.on_reducer(
			"create_standalone_channel",
			Box::new(move |ctx: &super::ReducerEventContext| {
				let super::ReducerEventContext {
					event:
						__sdk::ReducerEvent {
							reducer: super::Reducer::CreateStandaloneChannel { alias, metadata },
							..
						},
					..
				} = ctx
				else {
					unreachable!()
				};
				callback(ctx, alias, metadata)
			}),
		))
	}

	fn remove_on_create_standalone_channel(&self, callback: CreateStandaloneChannelCallbackId) {
		self.imp
			.remove_on_reducer("create_standalone_channel", callback.0)
	}
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer
/// `create_standalone_channel`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version
/// bump.
pub trait set_flags_for_create_standalone_channel {
	/// Set the call-reducer flags for the reducer `create_standalone_channel`
	/// to `flags`.
	///
	/// This type is currently unstable and may be removed without a major
	/// version bump.
	fn create_standalone_channel(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_create_standalone_channel for super::SetReducerFlags {
	fn create_standalone_channel(&self, flags: __ws::CallReducerFlags) {
		self.imp
			.set_call_reducer_flags("create_standalone_channel", flags);
	}
}

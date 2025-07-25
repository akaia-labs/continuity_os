use singularity_client::{
	common::{
		ports::{ProfileResolution, RecordResolver},
		presentation::Displayable,
		stdb::{ActorId, EventContext},
	},
	domain::{
		entities::{external_platform::SupportedExternalActorOrigin, message::MessageType},
		intersections::PlatformAssociation,
	},
};
use teloxide_core::types::{ChatId, MessageId, ThreadId};

use crate::integrations::{ports::SingularityMessage, telegram::OutboundTelegramMessage};

impl OutboundTelegramMessage {
	pub fn from_native(ctx: &EventContext, msg: &SingularityMessage) -> Self {
		let (author_role, author_profile) = match &msg.author {
			| ActorId::External(account_id) => account_id
				.resolve(ctx)
				.map_or((None, None), |account| (None, account.profile(ctx))),

			| ActorId::Internal(account_id) => account_id
				.resolve(ctx)
				.map(|account| {
					account
						.platform_association(ctx, SupportedExternalActorOrigin::Telegram)
						.map_or(
							(Some(account.role), account.profile(ctx)),
							|external_actor| (None, external_actor.profile(ctx)),
						)
				})
				.unwrap_or((None, None)),
		};

		let author_name = author_profile
			.map(|profile| profile.display_name())
			.unwrap_or(format!("unknown-{}", msg.sender));

		OutboundTelegramMessage {
			// TODO: `chat_id` and `thread_id` must be taken from MessageChannel
			chat_id:   ChatId(-1001544271932),
			thread_id: Some(ThreadId(MessageId(3315))),

			// TODO: must be taken from Message
			reply_to_message_id: None,

			text: format!(
				"{}\n\n{}",
				format!(
					"{} <strong>{author_name}</strong>",
					MessageType::symbol_by_account_role(author_role)
				),
				msg.text
			),
		}
	}
}

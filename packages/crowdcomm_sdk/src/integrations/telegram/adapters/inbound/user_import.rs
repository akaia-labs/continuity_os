use crate::{
	singularity::stdb::{
		ActorProfileMetadata, ActorName, ExternalActorOrigin, ExternalActorReference,
	},
	integrations::ports::{ExternalActorIdentification, ProfileImport, TelegramUser},
};

impl ExternalActorIdentification for TelegramUser {
	fn into_actor_ref(&self) -> ExternalActorReference {
		ExternalActorReference {
			id:     self.id.to_string(),
			origin: ExternalActorOrigin::Telegram,
		}
	}
}

impl ProfileImport for TelegramUser {
	fn into_actor_profile_metadata(&self) -> ActorProfileMetadata {
		ActorProfileMetadata {
			name: ActorName {
				short_name:     self.first_name.clone(),
				name_extension: self.last_name.clone(),
			},

			// TODO: Implement bio retrieval
			description: "".to_string(),
		}
	}
}

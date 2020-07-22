use crate::{
	entities::{
		Notification,
		Status,
	},
	streaming::{
		EventType,
	},
};
use std::result::Result as StdResult;
use std::error::Error as StdError;

pub trait EventHandler {
	type EventHandlingError: Into<Box<dyn StdError>>;

	fn handle(&self, event_type: EventType) -> StdResult<(), Self::EventHandlingError> {
        match event_type {
            EventType::Update(status) => {
				self.update(status.as_ref())
            },
            EventType::Notification(notification) => {
				self.notification(notification.as_ref())
            },
            EventType::Delete(status_id) => {
				self.delete(status_id)
            },
            EventType::FiltersChanged => {
				self.filters_changed()
            },
            EventType::Unknown(msg) => {
				self.unknown(msg)
            },
        }
	}

	fn update(&self, status: &Status) -> StdResult<(), Self::EventHandlingError> {
		println!("Status receved: {}", status.id());
		Ok(())
	}

	fn notification(&self, notification: &Notification) -> StdResult<(), Self::EventHandlingError> {
		println!("Notification raceived: {}", notification.id());
		Ok(())
	}

	fn delete(&self, deleted_status: impl AsRef<str>) -> StdResult<(), Self::EventHandlingError> {
		println!("Status deleted: {}", deleted_status.as_ref());
		Ok(())
	}

	fn filters_changed(&self) -> StdResult<(), Self::EventHandlingError> {
		println!("Filters changed");
		Ok(())
	}

	fn unknown(&self, msg: impl AsRef<str>) -> StdResult<(), Self::EventHandlingError> {
		println!("Unknown event received: {:#?}", msg.as_ref());
		Ok(())
	}
}

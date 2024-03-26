use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::helpers;

use super::data_models::Event;

#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_create_event", skip_on_field_errors = false))]
pub struct  CreateEvent {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 0))]
    pub price: i32,
    pub start_sale_date_time: DateTime<Utc>,
    pub end_sale_date_time: DateTime<Utc>,
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
}

fn validate_create_event(create_event:&&CreateEvent) -> Result<(), ValidationError> {
    if create_event.start_date_time > create_event.end_date_time {
        return Err(ValidationError::new("start_date_time must be before end_date_time"));
    }
    if create_event.start_sale_date_time > create_event.end_sale_date_time {
        return Err(ValidationError::new("start_sale_date_time must be before end_sale_date_time"));
    }
    if create_event.start_sale_date_time > create_event.start_date_time {
        return Err(ValidationError::new("start_date_time must be after start_sale_date_time"));
    }
    if create_event.start_date_time < helpers::get_current_time() {
        return Err(ValidationError::new("start_date_time must be in the future"));
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct CreateEventResult {
    pub id: String,
}

#[derive(Serialize)]
pub struct  EventDetails {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub start_sale_date_time: DateTime<Utc>,
    pub end_sale_date_time: DateTime<Utc>,
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
}

impl  From<&Event> for EventDetails {
    fn from(value: &Event) -> Self {
        let id = match  value.id {
            Some(object_id) => object_id.to_hex(),
            None => "".to_string(),
        };
        EventDetails {
            id,
            name: value.name.clone(),
            price: value.price,
            start_sale_date_time: value.start_sale_date_time,
            end_sale_date_time: value.end_sale_date_time,
            start_date_time: value.start_date_time,
            end_date_time: value.end_date_time,
        }
    }
}


#[cfg(test)]
mod validate_create_event_tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_validation_fails_on_empty_name() {
        let create_event = CreateEvent {
            name: "".to_string(),
            price: 0,
            start_sale_date_time: helpers::get_current_time(),
            end_sale_date_time: helpers::get_current_time(),
            start_date_time: helpers::get_current_time(),
            end_date_time: helpers::get_current_time(),
        };
        
        assert!(create_event.validate().is_err());
    }

    #[test]
    fn test_validation_fails_on_start_sale_after_end_sale() {
        let create_event = CreateEvent {
            name: "Event".to_string(),
            price: 0,
            start_sale_date_time: helpers::get_current_time(),
            end_sale_date_time: helpers::get_current_time() - Duration::days(1),
            start_date_time: helpers::get_current_time(),
            end_date_time: helpers::get_current_time(),
        };

        assert!(create_event.validate().is_err());
    }

    #[test]
    fn test_validation_fails_on_start_before_start_sale() {
        let create_event = CreateEvent {
            name: "Event".to_string(),
            price: 0,
            start_sale_date_time: helpers::get_current_time() + Duration::days(1),
            end_sale_date_time: helpers::get_current_time() + Duration::days(2),
            start_date_time: helpers::get_current_time(),
            end_date_time: helpers::get_current_time(),
        };

        assert!(create_event.validate().is_err());
    }

    #[test]
    fn test_validation_fails_on_start_in_the_past() {
        let create_event = CreateEvent {
            name: "Event".to_string(),
            price: 0,
            start_sale_date_time: helpers::get_current_time() - Duration::days(1),
            end_sale_date_time: helpers::get_current_time(),
            start_date_time: helpers::get_current_time() - Duration::days(2),
            end_date_time: helpers::get_current_time(),
        };

        assert!(create_event.validate().is_err());
    }

    #[test]
    fn test_validation_passes_on_valid_create_event() {
        let create_event = CreateEvent {
            name: "Event".to_string(),
            price: 0,
            start_sale_date_time: helpers::get_current_time() - Duration::days(1),
            end_sale_date_time: helpers::get_current_time(),
            start_date_time: helpers::get_current_time() + Duration::days(1),
            end_date_time: helpers::get_current_time() + Duration::days(2),
        };

        assert!(create_event.validate().is_ok());
    }
}




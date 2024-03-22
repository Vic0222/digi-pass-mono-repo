use super::{
    data_models::Event, data_transfer_objects::{CreateEvent, CreateEventResult, EventDetails}, event_repository::EventRepository
};

#[derive(Clone)]
pub struct EventManager {
    pub event_repository: Box<dyn EventRepository>,
}

impl EventManager {


    pub fn new(event_repository: Box<dyn EventRepository + Send + Sync>) -> Self {
        EventManager {
            event_repository: event_repository,
        }
    }

    pub async fn create_event(&self, data: CreateEvent) -> anyhow::Result<CreateEventResult> {
        //validate data
        let event = map_event(data);

        let id = self.event_repository.add(event).await?;
        Ok(CreateEventResult{
            id: id
        })
    }

    pub async fn list(&self) -> anyhow::Result<Vec<EventDetails>> {
        let events: Vec<EventDetails> = self.event_repository.list().await?.iter().map(|e| {e.into()}).collect();
        Ok(events)
    }

    pub async fn get_event(&self, event_id: &String) -> anyhow::Result<Option<EventDetails>> {
        let event_details = self.event_repository.get_event(event_id).await?
            .and_then(|event| Some((&event).into()));
        Ok(event_details)
    }
}

fn map_event(data: CreateEvent) -> Event {
    Event {
        id: None,
        name: data.name,
        price: data.price,
        start_sale_date_time: data.start_sale_date_time,
        end_sale_date_time: data.end_sale_date_time,
        start_date_time: data.start_date_time,
        end_date_time: data.end_date_time,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_map_event() {
        let now = Utc::now();
        let create_event_data = CreateEvent {
            name: "Test Event".to_string(),
            price: 0,
            start_sale_date_time: now,
            end_sale_date_time: now,
            start_date_time: now,
            end_date_time: now,
        };

        let event = map_event(create_event_data);

        assert_eq!(event.id, Option::None);
        assert_eq!(event.name, "Test Event".to_string());
        assert_eq!(event.start_sale_date_time, now);
        assert_eq!(event.end_sale_date_time, now);
        assert_eq!(event.start_date_time, now);
        assert_eq!(event.end_date_time, now);
    }
}

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryInJSON {
    pub id: String,
    pub title: String,
    pub body: String,
    pub isoTime: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>
}

impl From<Entry> for EntryInJSON {
    fn from(value: Entry) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            body: value.body,
            isoTime: value.iso_time,
            lat: value.lat,
            lon: value.lon
        }
    }
}

pub struct Entry {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub iso_time: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>
}

impl TryFrom<EntryInJSON> for Entry {
    type Error = ();
    fn try_from(value: EntryInJSON) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.parse().unwrap(),
            title: value.title,
            body: value.body,
            iso_time: value.isoTime,
            lat: value.lat,
            lon: value.lon
        })
    }
}
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PostEntry {
    pub title: String,
    pub body: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>
}

pub struct Entry {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub iso_time: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>
}

impl Entry {
    pub fn from_post_entry(post_entry: PostEntry, id: u32, iso_time: String) -> Self {
        Self {
            id,
            title: post_entry.title,
            body: post_entry.body,
            iso_time,
            lat: post_entry.lat,
            lon: post_entry.lon
        }
    }
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
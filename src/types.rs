use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct Tree {
    pub name_english: &'static str,
    pub name_latin: &'static str,
    pub typical_height_m: Option<u8>,
    pub identifiable_features: &'static str,
}

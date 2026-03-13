#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ViewId {
    uuid: uuid::Uuid,
}

impl ViewId {
    pub fn random() -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
        }
    }
}

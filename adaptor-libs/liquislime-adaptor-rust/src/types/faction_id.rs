use crate::{FromGameApi, ToGameApi};

#[derive(Debug, Clone, Copy)]
pub struct FactionId {
    pub id: u8,
}

impl FactionId {
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    pub fn index(self) -> usize {
        self.id as _
    }
}

impl FromGameApi for FactionId {
    type Api = u32;

    fn from_game_api(input: Self::Api) -> Self {
        Self { id: input as u8 }
    }
}

impl ToGameApi for FactionId {
    type Api = u32;

    fn to_game_api(&self) -> Self::Api {
        self.id as u32
    }
}

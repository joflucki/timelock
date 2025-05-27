use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct MessagePreview {}

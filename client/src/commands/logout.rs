use crate::utils;

pub fn logout() {
    utils::delete_keys().expect("Error deleting keys");
}

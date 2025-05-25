use crate::utils;

pub fn logout() {
    utils::delete_keys().expect("Error deleting keys");
    utils::delete_username().expect("Error deleting username");
}

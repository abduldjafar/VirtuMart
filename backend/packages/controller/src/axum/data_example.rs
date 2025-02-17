use serde_json::json;

pub(super) fn user_registered() -> String {
    serde_json::to_string_pretty(&json!({
        "status": "success",
        "data": {
            "user": {
                "email": "string",
                "password": "string",
                "role": "string",
                "username": "string"
            }
        }
    }))
    .unwrap()
}

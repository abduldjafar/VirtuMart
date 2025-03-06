#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use serde_json::json;

    macro_rules! api_test {
        ($name:ident, $method:ident, $endpoint:expr, $payload:expr) => {
            #[tokio::test]
            async fn $name() -> Result<()> {
                let hc = httpc_test::new_client("http://localhost:3000")?;
                
                let response = match stringify!($method) {
                    "GET" => hc.do_get($endpoint).await?,
                    "POST" => hc.do_post($endpoint, $payload).await?,
                    _ => panic!("Unsupported HTTP method"),
                };

                assert_eq!(response.status(), 200);

                Ok(())
            }
        };
    }

    api_test!(test_health, GET, "/api/health", json!({}));
    api_test!(test_swagger_run, GET, "/swagger-ui/", json!({}));
    api_test!(test_login, POST, "/api/v1/login", json!({
        "email": "test@example.com",
        "password": "password123"
    }));
    api_test!(test_register, POST, "/api/v1/user", json!({
        "email": "abdule@example.com",
        "password": "1234567890",
        "role": "admin",
        "username": "asoi909090"
    }));
}

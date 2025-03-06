#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use serde_json::json;

    #[tokio::test]
    async fn test_health() -> Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000")?;
        let response = hc.do_get("/api/health").await?;
        assert_eq!(response.status(), 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_swagger_run() -> Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000")?;
        let response = hc.do_get("/swagger-ui/").await?;
        assert_eq!(response.status(), 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_login() -> Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000")?;

        let payload = json!({
            "email": "test@example.com",
            "password": "password123"
        });

        let response = hc.do_post("/api/v1/login", payload).await?;

        println!("{:?}", response.json_body()?);
        assert_eq!(response.status(), 200);

        Ok(())
    }

    #[tokio::test]
    async fn test_register() -> Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000")?;

        let payload = json!({
          "email": "abdule@example.com",
          "password": "1234567890",
          "role": "admin",
          "username": "asoi909090"
        });

        let response = hc.do_post("/api/v1/user", payload).await?;

        println!("{:?}", response.json_body()?);
        assert_eq!(response.status(), 200);

        Ok(())
    }
}

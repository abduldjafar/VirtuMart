use environment::Environment;
use errors::Result;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Response, Surreal,
};

pub(super) async fn setup_direct_db() -> Result<Surreal<Client>> {
    let env = Environment::new();
    let client = Surreal::new::<Ws>(format!("{}:{}", env.db_host, env.db_port)).await?;
    client
        .signin(Root {
            username: &env.db_user,
            password: &env.db_pass,
        })
        .await?;
    client.use_ns(env.db_namespace).use_db(env.db_name).await?;
    Ok(client)
}

pub(super) async fn execute_sql(query: &str) -> Result<Response> {
    setup_direct_db()
        .await?
        .query(query)
        .await
        .map_err(|e| e.into())
}

pub(super) async fn cleanup_user(id: &str) -> Result<()> {
    setup_direct_db()
        .await?
        .query(&format!("DELETE FROM user WHERE id = {}", id))
        .await?;
    Ok(())
}

pub(super) async fn cleanup_data(id: &str, tb: &str) -> Result<()> {
    setup_direct_db()
        .await?
        .query(&format!("DELETE FROM {} WHERE id = {}", tb, id))
        .await?;
    Ok(())
}

#[macro_export]
macro_rules! setup_repo_with_surreal {
    ($fn_name:ident, $struct_name:ident, $field_name:ident) => {
        async fn $fn_name() -> Result<$struct_name> {
            let db_client = DatabaseClient::Surreal(SurrealDb {
                client: Some(setup_direct_db().await?),
            });
            Ok($struct_name {
                $field_name: std::sync::Arc::new(db_client),
            })
        }
    };
}

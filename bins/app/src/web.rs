use crate::middleware::auth::Auth;
use actix_web::{App, HttpServer};
use infrastructure::db::Pg;
use std::env;
use std::sync::Arc;

/// Start the server
#[cfg(not(tarpaulin_include))]
pub async fn start_server(pg: Arc<Pg>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // Session checker
            .wrap(Auth::new().add_ignored_routes(vec![
                "/auth/register".to_string(),
                "/auth/login".to_string(),
                "/auth/refresh".to_string(),
                "/auth/verify-email/{action_token}".to_string(),
                "/auth/verify-email/resend".to_string(),
                "/_health".to_string(),
            ]))
            .app_data::<infrastructure::state::AppState>(infrastructure::state::initialize())
            .configure(|cfg| crate::application::configure(Arc::clone(&pg), cfg))
    })
    .bind((env::var("HOST").unwrap(), 8080))?
    .run()
    .await
}

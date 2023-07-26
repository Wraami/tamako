mod api;
mod auth;
mod db;
mod templates;

#[api::main]
async fn main() -> tide::Result<()> {
    femme::start();
    dotenvy::dotenv().ok();

    let database = db::open().await?;
    let mut tamako = tide::with_state(database.clone());
    tamako.with(tide_compress::CompressMiddleware::new());

    tamako.at("/").get(templates::home);
    tamako.at("/auth").get(templates::auth);

    tamako.at("/api").nest({
        let mut api = tide::with_state(database);
        api.with(tide_compress::CompressMiddleware::new());

        api.at("/health").get(|_| async move { Ok("💚") });

        api.at("/whisper").get(api::list);
        api.at("/whisper/:snowflake").get(api::get);
        api.at("/whisper")
            .with(tide_governor::GovernorMiddleware::per_minute(2)?)
            .post(api::add);
        api.at("/whisper/:snowflake").delete(api::delete);

        api.at("/auth").post(api::auth);

        api
    });

    tamako.at("/assets").serve_dir("assets")?;

    let addr = (api::HOST.as_str(), *api::PORT);
    tamako.listen(addr).await?;

    Ok(())
}

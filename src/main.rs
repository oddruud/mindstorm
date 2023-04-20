#[shuttle_runtime::main]
async fn tide() -> shuttle_tide::ShuttleTide<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/hello").get(|_| async { Ok("Hello Ruud!!") });

    Ok(app.into())
}

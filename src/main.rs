use axum::{
    extract::State,
    headers::Expires,
    response::{IntoResponse, Redirect},
    Router, TypedHeader,
};
use axum_extra::routing::{RouterExt as _, TypedPath};
use modio::{Builder as ModioBuilder, Credentials, Modio};
use serde::Deserialize;
use std::time::{Duration, SystemTime};

#[derive(TypedPath, Deserialize)]
#[typed_path("/:game_name/:pak_name")]
struct PakUrl {
    game_name: String,
    pak_name: String,
}

#[derive(Clone)]
struct AppState {
    modio: Modio,
}

#[axum::debug_handler]
async fn get_pak(
    PakUrl {
        game_name,
        pak_name,
    }: PakUrl,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("get_pak: {game_name}/{pak_name}");

    let game_id = 1024;
    let mod_id = 10519;
    let file_id = 14391;

    let mod_ = state.modio.mod_(game_id, mod_id);
    let file = mod_.file(file_id);

    let file = file.get().await.unwrap();

    println!("file.download: {:?}", file.download);

    let expires = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(file.download.date_expires))
        .unwrap();
    println!("Expires: {:?}", expires);

    let expires = Expires::from(expires);

    (
        TypedHeader(expires),
        Redirect::temporary(file.download.binary_url.as_str()),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modio = ModioBuilder::new(Credentials::new("2318a53cb5bc72fe7b85ae51f2bc8a21"))
        .use_test()
        .build()?;

    let state = AppState { modio };

    let app = Router::new().typed_get(get_pak).with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

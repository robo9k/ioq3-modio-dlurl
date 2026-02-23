use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
};
use axum_extra::TypedHeader;
use axum_extra::routing::{RouterExt as _, TypedPath};
use headers::Expires;
use modio::types::id::{FileId, GameId, ModId};
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(TypedPath, Deserialize)]
#[typed_path("/{game_name}/{pak_name}")]
struct PakUrl {
    game_name: String,
    pak_name: String,
}

#[derive(Clone)]
struct AppState {
    modio: Arc<modio::Client>,
}

async fn get_pak(
    PakUrl {
        game_name,
        pak_name,
    }: PakUrl,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("get_pak: {game_name}/{pak_name}");

    let game_id = GameId::new(1024);
    let mod_id = ModId::new(10519);
    let file_id = FileId::new(14391);

    let file = state
        .modio
        .get_file(game_id, mod_id, file_id)
        .await
        .unwrap()
        .data()
        .await
        .unwrap();

    println!("file.download: {:?}", file.download);

    let expires = file.download.date_expires.as_secs().try_into().unwrap();

    let expires = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(expires))
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
    let modio = modio::Client::builder("0feae296118381c3bc38bab600291c96".to_string())
        .use_test_env()
        .build()?;

    let state = AppState {
        modio: modio.into(),
    };

    let app = Router::new().typed_get(get_pak).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}

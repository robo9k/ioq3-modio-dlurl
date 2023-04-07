use headers::Header as _;
use modio::{Builder as ModioBuilder, Credentials};
use std::time::{Duration, SystemTime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modio = ModioBuilder::new(Credentials::new("0xdeadbeef"))
        .use_test()
        .build()?;

    let game_id = 1024;
    let mod_id = 10519;
    let file_id = 14391;

    let mod_ = modio.mod_(game_id, mod_id);
    let file = mod_.file(file_id);

    let file = file.get().await?;

    println!("file.download: {:?}", file.download);

    let expires = SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(file.download.date_expires))
        .ok_or("nope")?;
    println!("Expires: {:?}", expires);

    let location = headers::Location::decode(
        &mut vec![headers::HeaderValue::from_str(
            file.download.binary_url.as_str(),
        )?]
        .iter(),
    )?;
    let expires = headers::Expires::from(expires);

    println!("http location: {:?}", location);
    println!("http expires: {:?}", expires);

    let mut location_values = vec![];
    let mut expires_values = vec![];
    location.encode(&mut location_values);
    expires.encode(&mut expires_values);

    println!("{}: {:?}", headers::Location::name(), location_values);
    println!("{}: {:?}", headers::Expires::name(), expires_values);

    Ok(())
}

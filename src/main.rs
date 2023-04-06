#[macro_use] extern crate rocket;
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePublicKey};

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

// Try visiting:
//   http://127.0.0.1:8000/hello/мир
#[get("/мир")]
fn mir() -> &'static str {
    "Привет, мир!"
}

// Try visiting:
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
//
// Try visiting:
//   http://127.0.0.1:8000/?emoji
//   http://127.0.0.1:8000/?name=Rocketeer
//   http://127.0.0.1:8000/?lang=ру
//   http://127.0.0.1:8000/?lang=ру&emoji
//   http://127.0.0.1:8000/?emoji&lang=en
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en
//   http://127.0.0.1:8000/?emoji&name=Rocketeer
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji
//   http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer
#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[launch]
fn rocket() -> _ {
    let priv_pem = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAi7+nQCxkT49GG2gwBs3D7f6aSKhuMSNH46tnqCv0V9xZouL6
aQAMCrP1aTsIDwrtcv5Ien+3mvx5w1s+aadKAylfAl2qL5V8rIUN5LfUfan/TFQF
ZLtcQSKguTFeU9KoVoXULdjzk9tBJJ27iW3HSuB2hPL4ebxlpHmJgQ/wFvv4uxgM
G/+rkcqKfIMark500meU7xoHk/w9wpRptMUuehoDFcy/TcNbQ/lKlUpk0nsqO9HM
DEghMbAeYFKbISkrBzHpids1Og0u/rIaj6ln57uyAfGYXUZlQatblRPnO8gOZb4U
jcCgdUKGvdz3TNNWy08uvDjtg+DFKRT3RSAZgQIDAQABAoIBABunlIhdgIhB+QSp
bH+cLn6lWJCyTAQLUEmcfMnJ0POmFlLmidlHehqQHCtz1Bdt+Sk9ashbB9FyzwG6
x7IOudXAVR3ZvTjnYyGcvvAY9MLdZjkLk3mongwBtz6Op3T9BfUBTMi52xHSXKTL
VfZPNBX8L1gF4OfuQUr4Bh71MXTKoGYHYQ8VfnICvRI77BDPWaJEcO3lVTEKVgRB
R5uIJILUwZ0l5G/G35H5+bYireeVbR3DUiAIbquS1vec1rQ0/zoGzzuYMQWNO9P6
F5rPz+AkvcFtDz4T7dd7NK8KwQ0qlRU7kheBVwFOAHzSzYGYF+qpgEiJIlquLpLE
Vt+G43UCgYEA+Ud9L6UAuzi1InP6LadmR22iJYtQiFHfXuukMlRn9tn4gchHQKti
hNvPBrTahYUNz0mPLWhE3rCiwWsL5l5Vx0CYzgufxuiz4WeFkZsniDIMpibBnyHI
FP4Z72PCZvT8bNamS7RyI+hoWAJmy+Pdy7CR4xWmWi0H0Iq5sZE9BOsCgYEAj4Qw
vZVTCbP/44sNQLUeVVqLdOkAxADN84jX8IkXZfRQZaZbwQtJdZGHLz8ysvt7g6o4
/TR+dYfA5t5Dj7CIgn1iTkZISfFzLIXdNe7kotqkuYAN6PHXp9HZOzHSrHl+TpkW
fr7oEYzQCmVBBk/WvWe8CIQtpnhp4IZf+jPDcEMCgYEA2keSRCUdNdPbMLcTq+R4
uDU1FeEe/K+MhAadOgEVJGALrkFcuzEXzLTZXMsCyoYO6KNF/4HdRI6xheKhgfC6
9gYHh+0/1KIx2T+YAz80Z5tD3j3zr8+Z71/t/+R/9z2wx/FRntr8U5sPu3km4ITA
07rilFGB4FasEB1ZqzY9k08CgYBM1Up61MzF7xXZrgIEWHLK41Sx3GRKitu3u9VP
szVRogDAytVofjBvvt+OzdjItZYkoSctd8MUDP+bGfc1oB8VLSdKHJpq69brSSdq
zmCdR0zU+td3lLIN6GzSIhRVvh1+2rqOVIpNiwN97UNoq6mdcUHblVgTPXgBei87
9iZ1nQKBgCG8b/vwXSeV6uRKJ5QpDIoWW0WG9qBJ4fuAzlB+OpOR1n5Ili8hNE03
phOp2QWM+JhBKWarexeZxgHyfAs5t5Ct92kB0CUMJZHX3J88OolOuH1Wra4WK7ny
0y9GeP0z9Bywqkkh+BlM4CFdxr61ozwuz2pExnbjIcaVsMnOX0+a
-----END RSA PRIVATE KEY-----";

    let pub_pem = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAi7+nQCxkT49GG2gwBs3D
7f6aSKhuMSNH46tnqCv0V9xZouL6aQAMCrP1aTsIDwrtcv5Ien+3mvx5w1s+aadK
AylfAl2qL5V8rIUN5LfUfan/TFQFZLtcQSKguTFeU9KoVoXULdjzk9tBJJ27iW3H
SuB2hPL4ebxlpHmJgQ/wFvv4uxgMG/+rkcqKfIMark500meU7xoHk/w9wpRptMUu
ehoDFcy/TcNbQ/lKlUpk0nsqO9HMDEghMbAeYFKbISkrBzHpids1Og0u/rIaj6ln
57uyAfGYXUZlQatblRPnO8gOZb4UjcCgdUKGvdz3TNNWy08uvDjtg+DFKRT3RSAZ
gQIDAQAB
-----END PUBLIC KEY-----";

    let private_key = RsaPrivateKey::from_pkcs1_pem(priv_pem).unwrap();
    let public_key = RsaPublicKey::from_public_key_pem(pub_pem).unwrap();
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir])
        .mount("/wave", routes![wave])
}

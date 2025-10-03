#[cfg(feature = "youtube")]
pub mod youtube;

pub trait Api<'a> {
    const URL: &'a str;
    fn url(&self) -> String;
}

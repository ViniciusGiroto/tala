mod ping;

pub use ping::*;

type Callback = dyn Send + for<'a> FnOnce(&'a mut optics::system::System);

pub struct RawRequest {
    pub(crate) func: Box<Callback>,
}

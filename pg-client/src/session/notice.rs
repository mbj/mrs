use crate::protocol::backend;

pub trait Sink {
    fn notice(&mut self, notice: &backend::NoticeResponse<'_>);
}

impl<F: FnMut(&backend::NoticeResponse<'_>)> Sink for F {
    fn notice(&mut self, notice: &backend::NoticeResponse<'_>) {
        self(notice);
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Discard;

impl Sink for Discard {
    fn notice(&mut self, _notice: &backend::NoticeResponse<'_>) {}
}

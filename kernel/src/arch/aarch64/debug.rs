use spin::MutexGuard;

pub struct Writer<'a> {
    log: MutexGuard<'a, Option<Log>>,
}

impl<'a> Writer<'a> {
    pub fn new() -> Writer<'a> {}
}

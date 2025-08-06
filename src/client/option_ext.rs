use anyhow::{Result, anyhow};
use backtrace::Backtrace;

pub trait OptionExt<T> {
    #[allow(unused)]
    fn ok(self) -> Result<T>;
    #[allow(unused)]
    fn okor(self, err: &'static str) -> anyhow::Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok(self) -> Result<T> {
        match self {
            Some(value) => Ok(value),
            None => {
                let bt = Backtrace::new();
                Err(anyhow!("called `Option::ok()` on a `None` value: {:?}", bt))
            },
        }
    }
    fn okor(self, err: &'static str) -> anyhow::Result<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(anyhow!("{}", err)),
        }
    }
}
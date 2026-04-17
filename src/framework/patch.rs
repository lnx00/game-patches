pub trait Patch: Send + Sync {
    fn name(&self) -> &'static str;
    fn config_key(&self) -> Option<&'static str> { None }

    fn init() -> Result<Box<dyn Patch>, String> where Self: Sized;

    fn apply(&mut self) -> Result<(), String>;
    fn revert(&mut self) -> Result<(), String>;
}
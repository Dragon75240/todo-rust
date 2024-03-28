pub trait Option {
    fn exec (&self, args: &[String]) -> Result<(), String>;
}
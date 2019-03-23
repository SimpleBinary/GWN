pub trait Reportable {
    fn position(&self) -> (u32, u32);
    fn message(&self) -> &String;
}
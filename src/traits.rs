pub trait Object {
    fn to_string(&self) -> String;
    fn get_object_id(&self) -> String;
    fn set_object_id(&mut self, object_id: String);
}
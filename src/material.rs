pub struct MaterialDefinition {

}

pub struct MaterialInstance {
}

impl MaterialDefinition {
  pub fn get_default(&self) -> MaterialInstance {
    MaterialInstance {}
  }
}

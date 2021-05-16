use valuable::*;

#[derive(Default, Debug)]
pub struct HelloWorld {
    pub id: u32,
}

pub static HELLO_WORLD_FIELDS: &[NamedField<'static>] = &[NamedField::new("id")];

impl Valuable for HelloWorld {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_named_fields(&NamedValues::new(
            HELLO_WORLD_FIELDS,
            &[Value::U32(self.id)],
        ));
    }
}

impl Structable for HelloWorld {
    fn definition(&self) -> StructDef<'_> {
        StructDef::new_static("HelloWorld", Fields::Named(HELLO_WORLD_FIELDS))
    }
}

use crate::{
    eng::{
        ecs::{
            components::{
                input::InputComponent,
                sprite::SpriteComponent,
                transform::TransformComponent
            }
        }
    }
};
use ron;
use specs::{
    error::NoError,
    prelude::*,
    saveload::{DeserializeComponents, SerializeComponents, U64Marker, U64MarkerAllocator}
};
use std::{
    fmt,
    io
};

#[derive(Debug)]
enum Combined {
    Ron(ron::ser::Error)
}

impl fmt::Display for Combined {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Combined::Ron(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<ron::ser::Error> for Combined {
    fn from(x: ron::ser::Error) -> Self {
        Combined::Ron(x)
    }
}

impl From<NoError> for Combined {
    fn from(e: NoError) -> Self {
        match e {}
    }
}

pub struct SerializeSystem<W: io::Write> {
    pub writer : W
}

impl<'a, W: io::Write> System<'a> for SerializeSystem<W> {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, SpriteComponent>,
        ReadStorage<'a, TransformComponent>,
        ReadStorage<'a, InputComponent>,
        ReadStorage<'a, U64Marker>
    );

    fn run(&mut self, (entities, sprites, transforms, inputs, markers): Self::SystemData) {
        let mut ser = ron::ser::Serializer::new(Some(Default::default()), true);
        SerializeComponents::<NoError, U64Marker>::serialize(
            &(&sprites, &transforms, &inputs),
            &entities,
            &markers,
            &mut ser
        ).unwrap_or_else(|e| eprintln!("Error: {}", e));
        self.writer.write_all(ser.into_output_string().as_bytes()).expect("Could not serialise the ecs world");
    }
}

pub struct DeserializeSystem<R: io::Read> {
    pub reader : R
}

impl<'a, R: io::Read> System<'a> for DeserializeSystem<R> {
    type SystemData = (
        Entities<'a>,
        Write<'a, U64MarkerAllocator>,
        WriteStorage<'a, SpriteComponent>,
        WriteStorage<'a, TransformComponent>,
        WriteStorage<'a, InputComponent>,
        WriteStorage<'a, U64Marker>
    );

    fn run(&mut self, (entities, mut alloc, sprites, transforms, inputs, mut markers): Self::SystemData) {
        use ron::de::Deserializer;

        let mut content: Vec<u8> = vec![];

        self.reader.read_to_end(&mut content).unwrap();

        if let Ok(mut de) = Deserializer::from_bytes(&content) {
            DeserializeComponents::<Combined, _>::deserialize(
                &mut(sprites, transforms, inputs),
                &entities,
                &mut markers,
                &mut alloc,
                &mut de
            ).unwrap_or_else(|e| eprintln!("Error: {}", e));
        }
    }
}

#![feature(phase)]


extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::EntityContainer;


world! {
	components Component;

	derived_traits PartialEq, Show;
}


pub type Component = u32;


#[test]
fn it_should_export_all_entities() {
	let mut entities = Entities::new();

	let entity_1 = Entity::new().with_component(5);
	let entity_2 = Entity::new().with_component(8);

	let id_1 = entities.add(entity_1);
	let id_2 = entities.add(entity_2);

	let entities = entities.export();

	assert_eq!(2, entities.len());

	for &(id, entity) in entities.iter() {
		if id == id_1 {
			assert_eq!(entity_1, entity);
		}
		else if id == id_2 {
			assert_eq!(entity_2, entity);
		}
		else {
			panic!("Unexpected id: {}", id);
		}
	}
}

#[test]
fn it_should_import_entities() {
	let mut entities = Entities::new();

	let entity_id = 5;
	entities.import(entity_id, Entity::new().with_component(8));

	assert_eq!(1, entities.components.len());
	assert_eq!(8, entities.components[entity_id]);
}

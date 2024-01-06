#![allow(unused)]
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use log;
use tracing;
use tracing::field::debug;


#[derive(Default)]
struct Resources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn add_resource_2<T: Any>(&mut self, resource: T) {
        self.resources
            .insert(resource.type_id(), Box::new(resource));
    }

    pub fn get_resource_2<T: Any>(&self) -> Option<&T> {
        if let Some(resource) = self.resources.get(&TypeId::of::<T>()) {
            resource.downcast_ref()
        } else {
            None
        }
    }

    pub fn get_resource_mut_2<T: Any>(&mut self) -> Option<&mut T> {
        if let Some(resource) = self.resources.get_mut(&TypeId::of::<T>()) {
            resource.downcast_mut()
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct World {
    // components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    components: HashMap<TypeId, Vec<Rc<RefCell<dyn Any>>>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
    more_resources: Resources,
}

impl World {
    pub fn add_resource<T: Any>(&mut self, resource: T) {
        tracing::debug!("Resource added!");
        tracing::info!("Resource added!");
        tracing::warn!("Resource added!");
        tracing::error!("Resource added!");

        self.resources
            .insert(resource.type_id(), Box::new(resource));
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        if let Some(resource) = self.resources.get(&TypeId::of::<T>()) {
            resource.downcast_ref()
        } else {
            None
        }
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        if let Some(resource) = self.resources.get_mut(&TypeId::of::<T>()) {
            resource.downcast_mut()
        } else {
            None
        }
    }

    pub fn register_component<T: Any>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }

    // not being used
    // pub fn create_entity(&mut self) {
    //     self.components.iter_mut().for_each(|(component_id, s)| {
    //         self.components.push();
    //     }); 
    // }

    pub fn with_component(&mut self, component: impl Any) -> &mut Self {
        let type_id = component.type_id();
        let components_values = self.components.get_mut(&type_id).unwrap();
        if let Some(last_component_value) = components_values.last_mut(){
            // why is he not pushing for every with_component?
            // this replaces the component, so cant have more than one value for a component
            *last_component_value = Rc::new(RefCell::new(component)); 
        } else{
            components_values.push(Rc::new(RefCell::new(component)));
        }
        self
    }

    pub fn remove_resource<T: Any>(&mut self) -> Option<Box<dyn Any>> {
        self.resources.remove(&TypeId::of::<T>())
    }

    pub fn add_resource_2<T: Any>(&mut self, resource: T) {
        self.more_resources.add_resource_2(resource);
    }

    pub fn get_resource_2<T: Any>(&self) -> Option<&T> {
        self.more_resources.get_resource_2::<T>()
    }

    pub fn get_resource_mut_2<T: Any>(&mut self) -> Option<&mut T> {
        self.more_resources.get_resource_mut_2::<T>()
    }

}

struct FpsResource(pub u32);

#[test]
pub fn add_resource() {
    let mut world = World::default();
    let fps = FpsResource(32);
    let fps_type_id = &fps.type_id();

    world.add_resource(fps);
    assert!(world.resources.contains_key(&fps_type_id));
}

#[test]
pub fn get_resource_immutably() {
    let mut world = World::default();
    let fps = FpsResource(32);

    world.add_resource(fps);

    let resource = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(resource.0, 32);
}

#[test]
pub fn get_resource_mut() {
    let mut world = World::default();
    let fps = FpsResource(32);

    world.add_resource(fps);

    {
        let mut resource = world.get_resource_mut::<FpsResource>().unwrap();
        resource.0 = 322;
        tracing::debug!("{}", resource.0);
    }

    let changed_resource = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(changed_resource.0, 322);
}

#[test]
pub fn remove_resource() {
    let mut world = World::default();
    let fps = FpsResource(32);

    world.add_resource(fps);
    {
        world.remove_resource::<FpsResource>();
    }

    let resource = world.get_resource::<FpsResource>();
    assert!(resource.is_none());
}

#[derive(Debug, PartialEq)]
struct Location(pub f32, pub f32);

#[derive(Debug, PartialEq)]
struct Size(pub f32);

#[test]
pub fn register_component() {
    let mut world = World::default();

    world.register_component::<Location>();
    world.components.get(&TypeId::of::<Location>()).unwrap();
}


#[test]
pub fn create_entity() {
    let mut world = World::default();
    world.register_component::<Location>();
    world.register_component::<Size>();
   

    // let location = world.components.get(&TypeId::of::<Location>()).unwrap();
    // let size = world.components.get(&TypeId::of::<Size>()).unwrap();

    world.with_component(Location(12.0, 32.4));
    world.with_component(Size(2.0));

    let location = &world.components.get(&TypeId::of::<Location>()).unwrap()[0];
    let location = location.borrow();
    let location = location.downcast_ref::<Location>().unwrap();

    let size = &world.components.get(&TypeId::of::<Size>()).unwrap()[0];
    let size = size.borrow();
    let size = size.downcast_ref::<Size>().unwrap();

    assert_eq!(*location, Location(12.0, 32.4));
    assert_eq!(*size, Size(2.0));
}

#[test]
pub fn query() {
    let mut world = World::default();
    world.register_component::<Location>();
    world.register_component::<Size>();
   
    world.with_component(Location(12.0, 32.4));
    world.with_component(Size(2.0));

    let location = &world.components.get(&TypeId::of::<Location>()).unwrap()[0];
    let location = location.borrow();
    let location = location.downcast_ref::<Location>().unwrap();

    let size = &world.components.get(&TypeId::of::<Size>()).unwrap()[0];
    let size = size.borrow();
    let size = size.downcast_ref::<Size>().unwrap();

    assert_eq!(*location, Location(12.0, 32.4));
    assert_eq!(*size, Size(2.0));
}

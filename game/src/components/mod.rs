use alloc::vec::Vec;
use crate::entity::Entity;
use spin::{Once, Mutex, MutexGuard};

pub mod movable;
pub mod char_drawable;

static SYSTEMS: Once<Mutex<Systems>> = Once::new();

pub fn systems() -> MutexGuard<'static, Systems> {
    SYSTEMS.call_once(|| {
        Mutex::new(Systems::new())
    }).lock()
}

pub struct Systems {
    removers: Vec<fn(&Entity)>,
}

impl Systems {
    fn new() -> Systems {
        Systems {
            removers: Vec::new(),
        }
    }

    pub fn register_remover(&mut self, remover: fn(&Entity)) {
        self.removers.push(remover);
    }

    pub fn removers(&self) -> impl Iterator<Item = &fn(&Entity)> {
        self.removers.iter()
    }
}

#[macro_export]
macro_rules! impl_system {
    ($component_ident: ident) => {
        
        static SYSTEM: spin::Once<spin::RwLock<System>> = spin::Once::new();

        fn create() -> spin::RwLock<System> {
            let res = spin::RwLock::new(System::new());
            systems().register_remover(|e| {
                lock_mut().remove(e)
            });
            res
        }

        pub fn lock() -> spin::RwLockReadGuard<'static, System> {
            SYSTEM.call_once(create).read()
        }

        pub fn lock_mut() -> spin::RwLockWriteGuard<'static, System> {
            SYSTEM.call_once(create).write()
        }

        pub fn add(component: $component_ident) {
            lock_mut().add(component);
        }

        pub fn get_component(e: &Entity) -> Option<$component_ident> {
            SYSTEM.call_once(create)
            .read()
            .lookup(e)
            .map(|c| *c)
        }

        pub struct System {
            components: alloc::vec::Vec<$component_ident>,
            map: hashbrown::HashMap<Entity, usize>,
        }

        impl System {
            fn new() -> System {
                System {
                    components: Vec::new(),
                    map: hashbrown::HashMap::new(),
                }
            }

            pub fn add(&mut self, component: $component_ident) {
                self.components.push(component);
                self.map.insert(component.entity, self.components.len() - 1);
            }

            pub fn remove(&mut self, e: &Entity) {
                if let Some(&index) = self.map.get(e) {

                    let last = self.components.len() - 1;
                    let last_entity = self.components[last].entity;

                    self.components[index as usize] = self.components[last];

                    self.map.insert(last_entity, index);
                    self.map.remove(e);
                }
            }

            pub fn lookup(&self, e: &Entity) -> Option<&$component_ident> {
                if let Some(&index) = self.map.get(e) {
                    return Some(&self.components[index]);
                }

                None
            }

            pub fn lookup_mut(&mut self, e: &Entity) -> Option<&mut $component_ident> {
                if let Some(&mut index) = self.map.get_mut(e) {
                    return Some(&mut self.components[index]);
                }

                None
            }

            pub fn components(&self) -> &[$component_ident] {
                &self.components
            }

            pub fn components_mut(&mut self) -> &mut [$component_ident] {
                &mut self.components
            }
        }
    }
}
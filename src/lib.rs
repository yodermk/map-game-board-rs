use std::sync::Arc;

pub trait BoardSpot {
    type Item;
    fn get_default() -> Self::Item;
}

// T is a game-specific type for whatever can be stored on a given territory
pub struct MapTerritory<T>
where T: Clone + BoardSpot {
    name: String,
    name_short: String,
    connections_to: Vec<Arc<MapTerritory<T>>>,
    current: T
}

impl<T: Clone + BoardSpot + BoardSpot<Item = T>> MapTerritory<T> {
    pub fn territory_builder(name: String, name_short: String) -> MapTerritory<T> {
        MapTerritory {
            name,
            name_short,
            connections_to: vec![],
            current: T::get_default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BoardSpot;

    #[derive(Clone)]
    struct IntThing {
        thing: i32
    }

    impl BoardSpot for IntThing {
        type Item = i32;

        fn get_default() -> Self::Item {
            42
        }
    }
    fn setup() {

    }
    #[test]
    fn default()
    {
        let ithing = IntThing::get_default();
        assert_eq!(ithing, 42);
    }
}

use std::sync::Arc;

pub trait BoardSpot {
    type Item;
}

// T is a game-specific type for whatever can be stored on a given territory
pub struct MapTerritory<T>
where T: Clone + BoardSpot + Default {
    name: String,
    name_short: String,
    connections_to: Vec<Arc<MapTerritory<T>>>,
    current: T
}

impl<T: Clone + BoardSpot + BoardSpot<Item = T> + Default> MapTerritory<T> {
    pub fn territory_builder(name: String, name_short: String) -> MapTerritory<T> {
        MapTerritory {
            name,
            name_short,
            connections_to: vec![],
            current: Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BoardSpot;

    #[derive(Clone, Default, PartialEq, Debug)]
    struct IntThing {
        thing: i32
    }

    impl BoardSpot for IntThing {
        type Item = i32;
    }
    fn setup() {

    }
    #[test]
    fn default()
    {
        let ithing = IntThing { ..Default::default()};
        assert_eq!(ithing, IntThing{thing: 0});
    }
}

use std::collections::HashMap;

use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use rayon::prelude::*;
use tracing::log::info;
use uuid::Uuid;

use crate::planet::direction::Direction;
use crate::planet::planet::Planet;
use crate::planet::resource::Resource;

pub struct MapGenerator {}

impl MapGenerator {
    /*

    Every planet has an movement difficulty, which defines how much energy you need, to move on this planet. The planets in the inner map (planets which are ((mapsize-1)/3) fields away from the edge) have a difficulty of 3. The planets in the mid map (planets which are ((mapsize-1)/3/2) fields away from the edge) have a difficulty of 2. All the other planets are in the outer map and have a difficulty of 1.

The location of the planets also defines which ressources they can contain. The outer map only contains cole. The mid map contains iron and gems and the inner map can contain gold and platin. The distribution of the resoure should be 80% coal, 10% iron, 5% gems, 3% gold and 2% platin.

Some planets of the map are not accessable. They are not part of the neighbours of the accessing planets or resources. They represent obstacles for the players, where they have to move around. Their location is also chosen randomly.
The relationship between planets is express as neighbour relation (Direction(North,West,South,East))
     */

    pub fn create_map(size: usize) -> Vec<Vec<Option<Planet>>> {
        let mut planets = vec![vec![Some(Planet::new(Uuid::new_v4(), 0)); size]; size];
        let mut rng = rand::thread_rng();
        for x in 0..size {
            for y in 0..size {
                let mut planet = Planet::new(Uuid::new_v4(), MapGenerator::get_movement_difficulty(size, x, y));
                if rng.gen_range(0..100) < 80 {
                    planet.resources = Some(MapGenerator::get_resources(size, x, y));
                }
                planets[x][y] = Some(planet);
            }
        }
        //Remove "size" amount of random planets from the map.
        for _ in 0..size {
            loop {
                let x = rng.gen_range(0..size);
                let y = rng.gen_range(0..size);
                if let Some(_) = &mut planets[x][y] {
                    planets[x][y] = None;
                    break;
                }
            }
        }

        //Nachbarschaften setzen.
        for x in 0..size {
            for y in 0..size {
                let mut neighbours: HashMap<Direction,Uuid> = HashMap::new();
                if x > 0 {
                    if let Some(planet) = &planets[x-1][y] {
                        neighbours.insert(Direction::WEST, planet.get_planet_id());
                    }
                }
                if x < size-1 {
                    if let Some(planet) = &planets[x+1][y] {
                        neighbours.insert(Direction::EAST, planet.get_planet_id());
                    }
                }
                if y > 0 {
                    if let Some(planet) = &planets[x][y-1] {
                        neighbours.insert(Direction::NORTH, planet.get_planet_id());
                    }
                }
                if y < size-1 {
                    if let Some(planet) = &planets[x][y+1] {
                        neighbours.insert(Direction::SOUTH, planet.get_planet_id());
                    }
                }
                if let Some(planet) = &mut planets[x][y] {
                    planet.set_neighbours(neighbours);
                }
            }
        }
        let planets_ref_structure: &Vec<Vec<std::option::Option<&Planet>>> = &planets.par_iter()
            .map(|row| {
                row.par_iter().map(|planet_opt| planet_opt.as_ref()).collect()
            })
            .collect();
        let map_print_as_string = MapGenerator::display_map_with_connections(planets_ref_structure);
        info!("Map:\n{}", map_print_as_string);
        return planets;
    }

    pub fn convert_to_hashmap(planets: &Vec<Vec<Option<Planet>>>) -> HashMap<Uuid, Planet> {
        let mut map = HashMap::new();
        for x in 0..planets.len() {
            for y in 0..planets[x].len() {
                if let Some(planet) = &planets[x][y] {
                    map.insert(planet.get_planet_id(), planet.clone());
                }
            }
        }
        map
    }

    pub fn display_map_with_connections(planets: &Vec<Vec<Option<&Planet>>>) -> String {
        let mut display_string = String::new();
        for y in 0..planets.len() {
            let mut connections_ns = String::new(); // Für vertikale Verbindungen

            for x in 0..planets[y].len() {
                // Zelle für den Planeten
                let cell = match &planets[x][y] {
                    Some(planet) => match &planet.resources {
                        Some((Resource::COAL, _)) => "C",
                        Some((Resource::IRON, _)) => "I",
                        Some((Resource::GEM, _)) => "Ge",
                        Some((Resource::GOLD, _)) => "Go",
                        Some((Resource::PLATINUM, _)) => "P",
                        None => "E", // Keine Ressource
                    },
                    None => "X", // Kein Planet
                };
                display_string.push_str(&format!(" {:^3} ", cell)); // Zentrieren Sie den Text im Feld

                // Horizontale Verbindung nach Osten
                if x < planets[y].len() - 1 {
                    display_string.push_str(if MapGenerator::planet_has_eastern_neighbor(planets, x, y) { "-" } else { " " });
                }
            }

            display_string.push('\n');

            // Vertikale Verbindung nach Süden
            if y < planets.len() - 1 {
                for x in 0..planets[y].len() {
                    connections_ns.push_str(if MapGenerator::planet_has_southern_neighbor(planets, x, y) { "  |  " } else { "     " });
                    if x < planets[y].len() - 1 {
                        connections_ns.push_str(" "); // Platz für die Trennung zwischen vertikalen Verbindungen
                    }
                }
                connections_ns.push('\n');
            }

            display_string.push_str(&connections_ns);
        }

        display_string
    }



    // Hilfsfunktionen, um zu prüfen, ob ein Planet östliche bzw. südliche Nachbarn hat
    fn planet_has_eastern_neighbor(planets: &Vec<Vec<Option<&Planet>>>, x: usize, y: usize) -> bool {
        if let Some(planet) = &planets[x][y] {
            if let Some(_) = planet.neighbours.get(&Direction::EAST) {
                return planets[x + 1][y].is_some();
            }
        }
        false
    }

    fn planet_has_southern_neighbor(planets: &Vec<Vec<Option<&Planet>>>, x: usize, y: usize) -> bool {
        if let Some(planet) = &planets[x][y] {
            if let Some(_) = planet.neighbours.get(&Direction::SOUTH) {
                return planets[x][y+1].is_some();
            }
        }
        false
    }

    fn get_movement_difficulty(size: usize, x: usize, y: usize) -> u8 {
        let mid_map = (size - 1) / 3 / 2;
        let inner_map = (size - 1) / 3;
        if x < mid_map || x > size - mid_map || y < mid_map || y > size - mid_map {
            return 1;
        }
        if x < inner_map || x > size - inner_map || y < inner_map || y > size - inner_map {
            return 2;
        }
        return 3;
    }

    fn get_resources(size: usize, x: usize, y: usize) -> (Resource,u32){
        let mut rng = rand::thread_rng();
        let mut dist = WeightedIndex::new(&[60, 20, 10, 7, 3]).unwrap();
        //convert usize to int from dist
        let mut resource = match dist.sample(&mut rng) {
            0 => Resource::COAL,
            1 => Resource::IRON,
            2 => Resource::GEM,
            3 => Resource::GOLD,
            4 => Resource::PLATINUM,
            _ => panic!("Error while generating resources")
        };
        (resource, 10000)
    }
}

#[cfg(test)]
mod tests {
    use crate::planet::map_generator::MapGenerator;

    #[test]
    fn test_create_map() {
        let size = 15;
        let map = MapGenerator::create_map(size);
        for row in map {
            assert_eq!(row.len(), size);
        }
    }

    #[test]
    fn test_get_movement_difficulty() {
        assert_eq!(MapGenerator::get_movement_difficulty(15, 0, 0), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 1, 0), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 2, 0), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 9, 9), 3);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 10, 9), 3);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 11, 9), 3);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 12, 10), 2);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 12, 11), 2);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 12, 12), 2);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 12, 13), 2);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 12, 14), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 12, 15), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 13, 15), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 14, 15), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 15, 15), 1);

        assert_eq!(MapGenerator::get_movement_difficulty(15, 9, 10), 3);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 9, 11), 3);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 10, 12), 2);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 11, 12), 2);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 13, 12), 2);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 14, 12), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 15, 12), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 15, 13), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 15, 14), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 15, 0), 1);
        assert_eq!(MapGenerator::get_movement_difficulty(15, 0, 15), 1);




    }
}

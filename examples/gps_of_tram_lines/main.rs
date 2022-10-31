pub mod model;
pub mod simplified;

use std::collections::HashMap;
use std::fs;
use geo_types::{coord, point, polygon, Geometry, GeometryCollection, Line, Point};
use geojson::FeatureCollection;
use geojson::{Feature, GeoJson};
use serde::Deserialize;

use model::{Element, OverpassTurboFile, OverpassTurbo};

#[derive(Deserialize)]
pub struct Ref {
    pub r#ref: String
}

fn generate_geo_json(x: &simplified::SimplifiedOverpassTurbo) {
    let mut geojson_data = Vec::new();
    let mut i = 0;
    for (key, element) in x.iter() {
        match element  {
            simplified::SimplifiedElement::Relation(relation) => {
                for position in &relation.members {
                    geojson_data.push(Geometry::from(Point(
                            coord! { x: position.lon, y: position.lat},
                    )));
                }
            }
            _ => {}
        };
    }

    let geometry_collection = GeometryCollection::from_iter(geojson_data);
    let feature_collection = FeatureCollection::from(&geometry_collection);

    fs::write("geo.json", serde_json::to_string(&feature_collection).unwrap());
}

fn main() {
    let mut y = OverpassTurbo::from_file("./tramdata.json").unwrap();

    for (key, value) in y.iter_mut() {
        match value {
            model::Element::Relation(rel) => {
                let mut new_members = Vec::new();
                for member in &rel.members {
                    if member.role != "stop" &&  member.role != "platform" {
                        new_members.push(member.clone());
                    }
                }
                rel.members = new_members.to_vec();
            }
            _ => {}
        }
    }

    let mut x = simplified::SimplifiedOverpassTurbo::from_struct(y);

    x.prune_nodes();
    x.prune_ways();
    
    // HashMap<String, Vec<Vec<(f32, f32)>> 
    let mut coords_by_line: HashMap<u32, Vec<Vec<(f64, f64)>>> = HashMap::new();

    for (key, value) in x.iter() {
        match value {
            simplified::SimplifiedElement::Relation(relation) => {
                let mut line;
                match serde_json::from_value::<Ref>(relation.tags.as_ref().unwrap().clone()) {
                    Ok(tags) => {
                        match tags.r#ref.parse::<u32>() {
                            Ok(number) => {
                                line = number;
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }

                let mut positions = Vec::new();

                for member in &relation.members {
                    positions.push((member.lat, member.lon));
                }

                match coords_by_line.get_mut(&line) {
                    Some(result) => {
                        result.push(positions);
                    }
                    None => {
                        coords_by_line.insert(line, vec![positions]);
                    }
                }
            }
            _ => {}
        }
    }
    fs::write("formatted.json", serde_json::to_string(&coords_by_line).unwrap());
}

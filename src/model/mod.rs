use serde_json;
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap};

/// struct containing meta information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenStreetMap {
    timestamp_osm_base: String,
    copyright: String
}

/// individual coordinate point with arbitrary
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Option<serde_json::Value>
}

/// relation between relation members
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relation {
    pub id: u64,
    pub members: Vec<RelationMember>,
    pub tags: Option<serde_json::Value>
}

/// contained in a relation ref is an id of some other element
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelationMember {
    pub r#type: String,
    pub r#ref: u64,
    pub role: String
}

/// the way is just a list of ids referencing other Nodes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Way {
    pub id: u64,
    pub nodes: Vec<u64>
}

/// different possible elements
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Element {
    #[serde(rename = "node")]
    Node(Node),

    #[serde(rename = "relation")]
    Relation(Relation),

    #[serde(rename = "way")]
    Way(Way)
}

impl Element {
    pub fn id(&self) -> u64 {
        match self {
            Element::Node(node) => node.id,
            Element::Way(way) => way.id,
            Element::Relation(relation) => relation.id
        }
    }
}

/// struct of an overpuss turbo json export
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverpassTurboFile {
    pub version: f32,
    pub generator: String,
    pub osm3s: OpenStreetMap,
    pub elements: Vec<Element>
}

impl OverpassTurboFile {
    /// reads a file into an overpass turbo file struct
    pub fn from_file(path: &str) -> Option<OverpassTurboFile> {
        match fs::read_to_string(path) {
            Ok(data) => {
                Some(serde_json::from_str::<OverpassTurboFile>(&data).unwrap())
            }
            Err(e) => {
                eprintln!("error: {:?}", e);
                None
            }
        }
    }
}


// handy struct stipped of all the meta information and in a 
// format where it is easier to look stuff up.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverpassTurbo(HashMap<u64, Element>);

impl OverpassTurbo {
    pub fn new() -> OverpassTurbo {
        OverpassTurbo(HashMap::new())
    }

    pub fn from_file(path: &str) -> Option<OverpassTurbo> {
        match OverpassTurboFile::from_file(path) {
            Some(loaded) => {
                let mut overpass_turbo = HashMap::new();
                for element in loaded.elements {
                    overpass_turbo.insert(element.id(), element);
                }

                Some(OverpassTurbo(overpass_turbo))
            }
            None => None
        }
    }
        // hashmap boilerplate
    pub fn iter(&self) -> impl Iterator<Item = (&u64, &Element)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&u64, &mut Element)> {
        self.0.iter_mut()
    }

    pub fn insert(&mut self, k: u64, v: Element) -> Option<Element> {
        self.0.insert(k, v)
    }

    pub fn get(&self, k: &u64) -> Option<&Element> {
        self.0.get(k)
    }

    pub fn empty() -> OverpassTurbo {
        OverpassTurbo(HashMap::new())
    }
}




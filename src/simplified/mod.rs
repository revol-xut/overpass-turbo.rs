use serde_json;
use serde::{Serialize, Deserialize, de::Error};
use std::{fs, collections::HashMap};

use crate::model::{Element, Node, Way, Relation, OverpassTurbo, OverpassTurboFile};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplifiedNode {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Option<serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplifiedRelation {
    pub id: u64,
    pub members: Vec<SimplifiedNode>,
    pub tags: Option<serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplifiedWay {
    pub id: u64,
    pub nodes: Vec<SimplifiedNode>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SimplifiedElement {
    #[serde(rename = "node")]
    Node(SimplifiedNode),

    #[serde(rename = "relation")]
    Relation(SimplifiedRelation),

    #[serde(rename = "way")]
    Way(SimplifiedWay)
}


impl SimplifiedNode {
    pub fn from(node: &Node) -> SimplifiedNode {
        SimplifiedNode {
            id: node.id,
            lat: node.lat,
            lon: node.lon,
            tags: node.tags.clone()
        }
    }
}

impl SimplifiedWay {
    pub fn from(way: &Way, overpass_turbo: &OverpassTurbo) -> SimplifiedWay {
        let mut vec = Vec::new();

        for node_id in &way.nodes {
            match SimplifiedElement::from(overpass_turbo.get(&node_id).unwrap(), overpass_turbo) {
                SimplifiedElement::Node(node) => {
                    vec.push(node);
                }
                _ => {}
            }

        }

       SimplifiedWay {
            id: way.id,
            nodes: vec
        }
    }
}

impl SimplifiedRelation {
    pub fn from(relation: &Relation, overpass_turbo: &OverpassTurbo) -> SimplifiedRelation {
        let mut vec = Vec::new();

        for relation_element in &relation.members {
            let elem = overpass_turbo.get(&relation_element.r#ref).unwrap();
            let mut new_nodes = match SimplifiedElement::from(elem, overpass_turbo) {
                SimplifiedElement::Way(way) => {
                    way.nodes
                }
                SimplifiedElement::Relation(relation) => {
                    relation.members
                }
                SimplifiedElement::Node(node) => {
                    vec![node]
                }
            };
            vec.append(&mut new_nodes);
        }

       SimplifiedRelation {
            id: relation.id,
            members: vec,
            tags: relation.tags.clone()
        }
    }
}

impl SimplifiedElement {
    pub fn id(&self) -> u64 {
        match self {
            SimplifiedElement::Node(node) => node.id,
            SimplifiedElement::Way(way) => way.id,
            SimplifiedElement::Relation(relation) => relation.id
        }
    }

    pub fn from(element: &Element, overpass_turbo: &OverpassTurbo) -> SimplifiedElement {
        match element {
            Element::Way(way) => {
                SimplifiedElement::Way(SimplifiedWay::from(&way, overpass_turbo))
            }
            Element::Node(node) => {
                SimplifiedElement::Node(SimplifiedNode::from(&node))
            }
            Element::Relation(relation) => {
                SimplifiedElement::Relation(SimplifiedRelation::from(&relation, overpass_turbo))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplifiedOverpassTurbo(HashMap<u64, SimplifiedElement>);

impl SimplifiedOverpassTurbo {
    pub fn from_path(path: &str) -> Option<SimplifiedOverpassTurbo> {
        match OverpassTurboFile::from_file(path) {
            Some(loaded) => Some(SimplifiedOverpassTurbo::from_file(loaded)),
            None => None
        }
    }

    pub fn from_file(overpass_turbo_file: OverpassTurboFile) -> SimplifiedOverpassTurbo {
        let mut overpass_turbo = OverpassTurbo::new();
        for element in overpass_turbo_file.elements {
            overpass_turbo.insert(element.id(), element);
        }
        SimplifiedOverpassTurbo::from_struct(overpass_turbo)
    }

    pub fn from_struct(overpass_turbo: OverpassTurbo) -> SimplifiedOverpassTurbo {
        let mut overpass_turbo_simplified = HashMap::new();

        for (key, value) in overpass_turbo.iter() {
            let simple_form = SimplifiedElement::from(&value, &overpass_turbo);

            overpass_turbo_simplified.insert(*key, simple_form);
        }

        SimplifiedOverpassTurbo(overpass_turbo_simplified)

    }



    pub fn prune_nodes(&mut self) {
        let mut delete_list = Vec::new();
        for (key, value) in self.iter() {
            match value {
                SimplifiedElement::Node(_) => {
                    delete_list.push(*key);
                }
                _ => {}
            }
        }

        for index in delete_list {
            self.remove(&index);
        }
    }

    pub fn prune_ways(&mut self) {
        let mut delete_list = Vec::new();
        for (key, value) in self.iter() {
            match value {
                SimplifiedElement::Way(_) => {
                    delete_list.push(*key);
                }
                _ => {}
            }
        }

        for index in delete_list {
            self.remove(&index);
        }
    }


    pub fn iter(&self) -> impl Iterator<Item = (&u64, &SimplifiedElement)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&u64, &mut SimplifiedElement)> {
        self.0.iter_mut()
    }

    pub fn insert(&mut self, k: u64, v: SimplifiedElement) -> Option<SimplifiedElement> {
        self.0.insert(k, v)
    }

    pub fn get(&self, k: &u64) -> Option<&SimplifiedElement> {
        self.0.get(k)
    }

    pub fn remove(&mut self, k: &u64) {
        self.0.remove(k);
    }
}




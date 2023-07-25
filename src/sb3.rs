use std::collections::HashMap;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct SB3 {
    targets: Vec<Target>,
    monitors: Vec<Monitor>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {}

#[derive(Deserialize, Debug)]
pub enum MonitorMode {
    Default,
    Large,
    Slider,
    List,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    is_stage: bool,
    name: String,
    variables: HashMap<String, Variable>,
    lists: HashMap<String, List>,
    // broadcasts could maybe be in sprites
    blocks: HashMap<String, Block>,
    comments: HashMap<String, Comment>,
    current_costume: usize,
    costumes: Vec<Costume>,
    sounds: Vec<Sound>,
    layer_order: usize,
    volume: f64,
}

#[derive(Deserialize, Debug)]
pub struct Variable {
    name: String,
    #[serde(deserialize_with = "boolean")]
    is_cloud: bool,
}

#[derive(Deserialize, Debug)]
pub struct List {
    name: String,
    value: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    opcode: String,
    next: Option<String>,
    parent: Option<String>,
    inputs: HashMap<String, Input>,
    fields: HashMap<String, Field>,
    shadow: bool,
    top_level: bool,
    x: Option<f64>,
    y: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Input {
    shadow: InputShadowStatus,
    val: (i32, String), // FIXME
}

#[derive(Debug)]
pub enum InputValue {
    Number(f64),
    PositiveNumber(f64),
    PositiveInteger(i64),
    Integer(i64),
    Angle(f64),
    Color(Color),
    String(String),
    Broadcast {
        name: String,
        id: String,
    },
    Variable {
        name: String,
        id: String,
        x: Option<f64>,
        y: Option<f64>,
    },
    List {
        name: String,
        id: String,
        x: Option<f64>,
        y: Option<f64>,
    },
}

// impl Deserialize for InputValue {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         // "inputs": { "VALUE": [1, [10, "4"]] },
//         // "inputs": { "VALUE": [1, [4, "1"]] },

//         let value = Value::deserialize(deserializer)?;

//         let mut iter = value
//             .as_array()
//             .ok_or(de::Error::custom("Invalid array"))?
//             .iter();

//         let input_type = iter
//             .next()
//             .ok_or(de::Error::custom("Invalid array"))?
//             .as_i64()
//             .ok_or(de::Error::custom("Invalid number"))?;

//         match input_type {
//             4 => {
//                 // number
//                 let number = iter
//                     .next()
//                     .ok_or(de::Error::custom("Invalid array"))?
//                     .as_f64()
//                     .ok_or(de::Error::custom("Invalid number"))?;
//                 return number;
//             }
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Deserialize, Debug)]
pub enum InputShadowStatus {
    Shadow,
    NoShadow,
    ObscuredShadow,
}

#[derive(Deserialize, Debug)]
pub struct Field {}
#[derive(Deserialize, Debug)]
pub struct Comment {}
#[derive(Deserialize, Debug)]
pub struct Costume {}
#[derive(Deserialize, Debug)]
pub struct Sound {}

fn boolean<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? != 0,
        Value::Null => false,
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    })
}

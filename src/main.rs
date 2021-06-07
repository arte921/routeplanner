use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Afstand {
    afstand: f32,
    van: String,
    naar: String,
}

struct Route {
    afstand: f32,
    route: Vec<Station>
}

struct Station {
    code: String,
    buren: Vec<Stationafstand>,
    bezocht: [bool; 2]
}

struct Stationafstand {
    afstand: f32,
    station: String,
}
/*
fn onderzoek_station(
    huidigstation: String,
    doelstation: String,
    huidigeafstand: f32,
    maximumafstand: f32,
    afstanden: &Vec<Afstand>,
    omliggendestationscollectie: &mut Vec<Stationafstand>,
) -> Option<Vec<String>> {
    if huidigstation == doelstation {
        return Some(gepasseerdestations);
    }

    let omliggendestations = afstanden
        .iter()
        .filter(|feature| feature.van == huidigstation || feature.naar == huidigstation)
        .map(|feature| Stationafstand {
            afstand: feature.afstand,
            station: if feature.van == huidigstation {
                feature.naar
            } else {
                feature.van
            },
        });
}

fn voeg_toe_aan_collectie(
    afstanden: &Vec<Afstand>,
    omliggendestationscollectie: &mut Vec<Stationafstand>,
) {

}

// lazy evaluated recursion? */

fn sla_feature_op_in_stations(stations: &mut Vec<Station>, stationa: String, stationb: String, afstand: f32) {
    match (stations).into_iter().find(|station| station.code == stationa) {
        Some(station) => station.buren.push(Stationafstand {
            station: stationa,
            afstand: afstand
        }),
        None => stations.push(Station {
            code: stationa,
            buren: vec![Stationafstand {
                station: stationb,
                afstand: afstand
            }],
            bezocht: [false, false]
        })
    }
}

fn main() -> std::io::Result<()> {
    let bestemmingen = ["ah", "asd"];

    let mut afstanden_json = String::new();
    File::open("opslag/featureafstanden.json")?.read_to_string(&mut afstanden_json)?;
    let afstanden: Vec<Afstand> = serde_json::from_str(&afstanden_json)?;

    let mut stations: Vec<Station> = Vec::new();

    for feature in afstanden {
        sla_feature_op_in_stations(&mut stations, feature.van.clone(), feature.naar.clone(), feature.afstand);
        sla_feature_op_in_stations(&mut stations, feature.van.clone(), feature.naar.clone(), feature.afstand);
    }

    let mut routes: Vec<Route> = vec![];
    

    println!("{}", stations.len());

    Ok(())
}2

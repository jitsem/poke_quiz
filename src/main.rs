use clap::Parser;
use rand::prelude::SliceRandom;
use rand::Rng;
use serde_json::Value;
use std::io::{self, BufRead};

mod poke_cache;

const MAX_POKEMON: u32 = 905;

/// Simple program to check pokemon data
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ID of the pokemon to search
    #[arg(short, long, default_value_t = 0)]
    id: u32,
}
fn main() {
    let args = Args::parse();
    if args.id > 0 {
        print_one_pokemon(args.id);
    }
    start_quiz_mode();
}

#[derive(Debug)]
struct PokeInfo {
    name: String,
    info: String,
}

fn start_quiz_mode() {
    let stdin = io::stdin();
    let mut score = 0;
    loop {
        let id = get_valid_dex_number();
        let poke_res = get_poke_description(id);
        if let Some(poke) = poke_res {
            println!();
            println!("Who's that pokemon?");
            println!("Tip: {}", poke.info);
            println!("Guess:");
            let answer = stdin.lock().lines().next().unwrap().unwrap();
            if poke.name.to_lowercase() == answer.to_lowercase() {
                println!("Correct!");
                score += 1;
                println!("Current sore: {}", score);
            } else {
                println!("Wrong, it was {}", poke.name);
                println!("Game over!!!");
                println!("Final sore: {}", score);
                break;
            }
        } else {
            panic!("MissingNo");
        }
    }
}

fn print_one_pokemon(entered_id: u32) {
    println!("Searching, {}!", entered_id);

    let poke_res = get_poke_description(entered_id);
    if let Some(poke) = poke_res {
        println!("Name: {}", poke.name);
        println!("Info: {}", poke.info);
    } else {
        println!("No such pokemon");
    }
}

fn get_poke_description(id: u32) -> Option<PokeInfo> {
    let text = get_poke_info(id)?;
    let eng_entries = parse_api_result(&text)?;
    let choice = eng_entries.choose(&mut rand::thread_rng())?;
    let formatted = choice.info.replace('\n', " ");
    Some(PokeInfo {
        name: capitalize_first_letter(&choice.name),
        info: formatted,
    })
}

fn parse_api_result(text: &str) -> Option<Vec<PokeInfo>> {
    let v: Value = serde_json::from_str(text).ok()?;
    let flavour_entries = v["flavor_text_entries"].as_array()?.iter();
    let eng_entries: Vec<PokeInfo> = flavour_entries
        .filter_map(|e| {
            return match e.as_object() {
                Some(o) => match o["language"].as_object() {
                    Some(lang) => match lang["name"].as_str() {
                        Some(str) => {
                            if str == "en" {
                                let en_info = e.as_object()?["flavor_text"].as_str()?.to_owned();
                                let en_name = v["name"].as_str()?.to_string();
                                return Some(PokeInfo {
                                    name: en_name,
                                    info: en_info,
                                });
                            }
                            None
                        }
                        None => None,
                    },
                    None => None,
                },
                None => None,
            };
        })
        .collect();
    Some(eng_entries)
}

fn get_poke_info(id: u32) -> Option<String> {
    let text: String;
    if let Some(res) = poke_cache::get_cached_response_for_id(id) {
        //println!("Found in cache");
        text = res;
    } else {
        //println!("Did not find in cache, querying API");
        let response = reqwest::blocking::get(
            "https://pokeapi.co/api/v2/pokemon-species/".to_string() + &id.to_string(),
        )
        .unwrap();
        text = response.text().ok()?;
        poke_cache::set_cached_response_for_id(id, &text);
    }
    Some(text)
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn get_valid_dex_number() -> u32 {
    rand::thread_rng().gen_range(0..=MAX_POKEMON)
}

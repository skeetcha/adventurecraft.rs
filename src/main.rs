use std::collections::HashMap;
use colored::Colorize;
use regex::Regex;
use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom};
use text_io::read;

fn main() -> AdventureResult<()> {
    let mut adventure = Adventure::new();
    adventure.run()
}

type Command = Box<dyn Fn(&mut bool, Vec<String>) -> Vec<String>>;

fn create_command(f: impl Fn(&mut bool, Vec<String>) -> Vec<String> + 'static) -> Command {
    Box::new(f)
}

type AdventureResult<T, E = String> = std::result::Result<T, E>;

struct Adventure {
    commands: HashMap<&'static str, Command>,
    matches: HashMap<&'static str, Vec<&'static str>>,
    command_history: Vec<String>,
    running: bool,
    room_map: HashMap<i32, HashMap<i32, HashMap<i32, Room>>>
}

impl Adventure {
    fn new() -> Self {
        Self {
            commands: Self::setup_commands(),
            matches: Self::setup_matches(),
            command_history: Vec::new(),
            running: true,
            room_map: HashMap::new()
        }
    }

    fn run(&mut self) -> AdventureResult<()> {
        self.do_command(String::from("look"))?;
        self.simulate();

        while self.running {
            print!("{}", "? ".yellow());

            let line: String = read!("{}\n");
            self.command_history.push(line.clone());
            self.do_command(line)?;
            self.simulate();
        }

        Ok(())
    }

    fn do_command(&mut self, text: String) -> AdventureResult<Vec<String>> {
        if text == "" {
            match self.commands.get("noinput") {
                Some(command) => {
                    return Ok(command(&mut self.running, Vec::new()));
                },
                None => {
                    return Err(String::from("noinput command not found"));
                }
            }
        }

        for (command, t) in &self.matches {
            for s_match in t.iter() {
                match Regex::new(&format!("^{}$", s_match)) {
                    Ok(re) => {
                        match re.captures(&text) {
                            Some(captures) => {
                                match self.commands.get(command) {
                                    Some(fn_command) => {
                                        if captures.len() == 1 {
                                            match captures.get(0) {
                                                Some(capture) => {
                                                    if String::from(capture.as_str()) == String::from(*s_match) {
                                                        return Ok(fn_command(&mut self.running, Vec::new()));
                                                    }
                                                },
                                                None => {
                                                    return Err(String::from("error getting capture"));
                                                }
                                            }
                                        } else {
                                            return Ok(fn_command(&mut self.running, captures.iter().map(|val| val.unwrap().as_str().into()).collect::<Vec<String>>()));
                                        }
                                    },
                                    None => {
                                        return Err(format!("{} command not found", command));
                                    }
                                }
                            },
                            None => ()
                        }
                    },
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }
            }
        }

        match self.commands.get("badinput") {
            Some(command) => Ok(command(&mut self.running, Vec::new())),
            None => Err(String::from("badinput command not found"))
        }
    }

    fn setup_matches() -> HashMap<&'static str, Vec<&'static str>> {
        let mut matches: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

        matches.insert("wait", vec!["wait"]);

        matches.insert("look", vec![
            "look at the ([a-zA-Z ]+)",
            "look at ([a-zA-Z ]+)",
            "look",
            "inspect ([a-zA-Z ]+)",
            "inspect the ([a-zA-Z ]+)",
            "inspect"
        ]);

        matches.insert("inventory", vec![
            "check self",
            "check inventory",
            "inventory",
            "i"
        ]);

        matches.insert("go", vec![
            "go ([a-zA-Z]+)",
            "travel ([a-zA-Z]+)",
            "walk ([a-zA-Z]+)",
            "run ([a-zA-Z]+)",
            "go"
        ]);

        matches.insert("dig", vec![
            "dig ([a-zA-Z]+) using ([a-zA-Z ]+)",
            "dig ([a-zA-Z]+) with ([a-zA-Z ]+)",
            "dig ([a-zA-Z]+)",
            "dig"
        ]);

        matches.insert("take", vec![
            "pick up the ([a-zA-Z ]+)",
            "pick up ([a-zA-Z ]+)",
            "pickup ([a-zA-Z ]+)",
            "take the ([a-zA-Z ]+)",
            "take ([a-zA-Z ]+)",
            "take"
        ]);

        matches.insert("drop", vec![
            "put down the ([a-zA-Z ]+)",
            "put down ([a-zA-Z ]+)",
            "drop the ([a-zA-Z ]+)",
            "drop ([a-zA-Z ]+)",
            "drop"
        ]);

        matches.insert("place", vec![
            "place the ([a-zA-Z ]+)",
            "place ([a-zA-Z ]+)",
            "place"
        ]);

        matches.insert("cbreak", vec![
            "punch the ([a-zA-Z ]+)",
            "punch ([a-zA-Z ]+)",
            "punch",
            "break the ([a-zA-Z ]+) with the ([a-zA-Z ]+)",
            "break ([a-zA-Z ]+) with ([a-zA-Z ]+)",
            "break the ([a-zA-Z ]+)",
            "break ([a-zA-Z ]+)",
            "break"
        ]);

        matches.insert("mine", vec![
            "mine the ([a-zA-Z ]+) with the ([a-zA-Z ]+)",
            "mine ([a-zA-Z ]+) with ([a-zA-Z ]+)",
            "mine ([a-zA-Z ]+)",
            "mine"
        ]);

        matches.insert("attack", vec![
            "attack the ([a-zA-Z ]+) with the ([a-zA-Z ]+)",
            "attack ([a-zA-Z ]+) with ([a-zA-Z ]+)",
            "attack ([a-zA-Z ]+)",
            "attack",
            "kill the ([a-zA-Z ]+) with the ([a-zA-Z ]+)",
            "kill ([a-zA-Z ]+)",
            "kill",
            "hit the ([a-zA-Z ]+) with the ([a-zA-Z ]+)",
            "hit ([a-zA-Z ]+) with ([a-zA-Z ]+)",
            "hit ([a-zA-Z ]+)",
            "hit"
        ]);

        matches.insert("craft", vec![
            "craft a ([a-zA-Z ]+)",
            "craft some ([a-zA-Z ]+)",
            "craft ([a-zA-Z ]+)",
            "craft",
            "make a ([a-zA-Z ]+)",
            "make some ([a-zA-Z ]+)",
            "make ([a-zA-Z ]+)",
            "make"
        ]);

        matches.insert("build", vec![
            "build ([a-zA-Z ]+) out of ([a-zA-Z ]+)",
            "build ([a-zA-Z ]+) from ([a-zA-Z ]+)",
            "build ([a-zA-Z ]+)",
            "build"
        ]);

        matches.insert("eat", vec![
            "eat a ([a-zA-Z ]+)",
            "eat the ([a-zA-Z ]+)",
            "eat ([a-zA-Z ]+)",
            "eat"
        ]);

        matches.insert("help", vec![
            "help me",
            "help"
        ]);

        matches.insert("exit", vec![
            "exit",
            "quit",
            "goodbye",
            "good bye",
            "bye",
            "farewell"
        ]);

        matches.to_owned()
    }

    fn setup_commands() -> HashMap<&'static str, Command> {
        let mut commands: HashMap<&'static str, Command> = HashMap::new();

        commands.insert("badinput", create_command(|_, _| {
            let responses = vec![
                "I don't understand.",
                "I don't understand you.",
                "You can't do that.",
                "Nope.",
                "Huh?",
                "Say again?",
                "That's crazy talk.",
                "Speak clearly.",
                "I'll think about it.",
                "Let me get back to you on that one.",
                "That doesn't make any sense.",
                "What?"
            ];

            println!("{}", match responses.choose(&mut rand::thread_rng()) {
                Some(r) => r,
                None => ""
            });

            Vec::new()
        }));

        commands.insert("noinput", create_command(|_, _| {
            let responses = vec![
                "Speak up.",
                "Enunciate.",
                "Project your voice.",
                "Don't be shy.",
                "Use your words."
            ];

            println!("{}", match responses.choose(&mut rand::thread_rng()) {
                Some(r) => r,
                None => ""
            });

            Vec::new()
        }));

        commands.insert("exit", create_command(|running, _| {
            *running = false;
            Vec::new()
        }));

        commands.insert("look", create_command(|_, args| {
            let target = args[0].clone();

            Vec::new()
        }));

        commands
    }

    fn simulate(&mut self) {

    }

    fn get_room(&mut self, x: i32, y: i32, z: i32, dont_create: bool) -> Room {
        let x_val = if let Some(val) = self.room_map.get_mut(&x) {
            val
        } else {
            &mut self.room_map.insert(x, HashMap::new()).unwrap()
        };

        let y_val = if let Some(val) = x_val.get_mut(&y) {
            val
        } else {
            &mut x_val.insert(y, HashMap::new()).unwrap()
        };

        let mut room: Room;

        match y_val.get(&z) {
            Some(_) => {

            },
            None => {
                if !dont_create {
                    room = y_val.insert(z, Room::default()).unwrap().clone();

                    if y == 0 {
                        room.biome = rand::random();
                        room.trees = room.biome.has_trees();
                    }
                }
            }
        }

        room
    }
}

#[derive(Default, Clone)]
struct Room {
    biome: Biome,
    trees: bool,
    items: Vec<Item>,
    exits: Exits,
    dark: bool,
    monsters: i32,
    valid: bool
}

impl Room {
    fn get_exits(&self) -> Vec<&'static str> {
        let mut exits = Vec::new();

        if self.exits.north {
            exits.push("north");
        }

        if self.exits.south {
            exits.push("south");
        }

        if self.exits.west {
            exits.push("west");
        }

        if self.exits.east {
            exits.push("east");
        }

        if self.exits.up {
            exits.push("up");
        }

        if self.exits.down {
            exits.push("down");
        }

        exits
    }
}

#[derive(Default, Clone)]
enum Biome {
    #[default]
    None,
    Forest,
    PineForest,
    Swamp,
    Mountain,
    Desert,
    Plain,
    Tundra
}

impl Into<&'static str> for Biome {
    fn into(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Forest => "in a forest",
            Self::PineForest => "in a pine forest",
            Self::Swamp => "knee deep in a swamp",
            Self::Mountain => "in a mountain range",
            Self::Desert => "in a desert",
            Self::Plain => "in a grassy plain",
            Self::Tundra => "in a frozen tundra"
        }
    }
}

impl Distribution<Biome> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Biome {
        match rng.gen_range(0..=6) {
            0 => Biome::Forest,
            1 => Biome::PineForest,
            2 => Biome::Swamp,
            3 => Biome::Mountain,
            4 => Biome::Desert,
            5 => Biome::Plain,
            _ => Biome::Tundra
        }
    }
}

impl Biome {
    fn has_trees(&self) -> bool {
        match self {
            Self::Forest => true,
            Self::PineForest => true,
            Self::Swamp => true,
            _ => false
        }
    }
}

#[derive(Clone)]
struct Item {
    droppable: bool,
    desc: String,
    heavy: bool,
    creature: bool,
    drops: Vec<String>,
    aliases: Vec<String>,
    hit_drops: Vec<String>,
    monster: bool,
    nocturnal: bool,
    material: bool,
    tool: bool,
    tool_level: Option<i32>,
    tool_type: Option<ToolType>,
    ore: bool,
    infinite: bool,
    food: bool
}

#[derive(Clone)]
enum ToolType {
    None,
    Pick,
    Sword,
    Shovel
}

#[derive(Default, Clone)]
struct Exits {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    down: bool,
    up: bool
}

impl Exits {
    fn get_exit(&self, s: String) -> bool {
        match s.as_ref() {
            "north" => self.north,
            "south" => self.south,
            "west" => self.west,
            "east" => self.east,
            "up" => self.up,
            "down" => self.down,
            _ => false
        }
    }

    fn set_exit(&mut self, s: String, v: bool) {
        match s.as_ref() {
            "north" => self.north = v,
            "south" => self.south = v,
            "west" => self.west = v,
            "east" => self.east = v,
            "up" => self.up = v,
            "down" => self.down = v,
            _ => ()
        }
    }
}
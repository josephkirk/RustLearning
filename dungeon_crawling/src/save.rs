#[derive(Debug)]
enum GameInput {
    Option,
    Quit,
    Left,
    Right,
    Up,
    Down,
    Attack,
    Jump,
    None
}

#[derive(Debug)]
enum Input {
    KeyboardInput(String),
    GamepadInput(String),
}

#[derive(Debug)]
enum CharacterStrait {
    Mad,
    Angry,
    Crazy,
    Silly
}

#[derive(Debug)]
enum CharacterStat {
    Strength(i32),
    Defence(i32),
    Luck(i32),
}


impl Input {
    fn check_input(input: &Input) -> GameInput {
        match input {
            Input::KeyboardInput(_input) => {
                let raw_input = &_input[..];
                // println!("{}", raw_input);
                match raw_input {
                    "option" | "o" | "O" | "Option" => GameInput::Option,
                    "quit" | "q" | "Quit" | "Q" => GameInput::Quit,
                    "A" | "a" => GameInput::Left,
                    "W" | "w" => GameInput::Up,
                    "S" | "s" => GameInput::Down,
                    "D" | "d" => GameInput::Right,
                    "J" | "j" => GameInput::Jump,
                    "K" | "k" => GameInput::Attack,
                    _ => GameInput::None,
                }
            },
            Input::GamepadInput(_input) => GameInput::None,
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    equipment: Vec<Equipment>,
    straits: Vec<CharacterStrait>,
    stats: Vec<CharacterStat>,
}
impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            equipment: Vec::new(),
            straits: Vec::new(),
            stats: Vec::new()
        }
    }
    
    fn add_trait(&self, _trait: CharacterTrait) -> 
}
#[derive(Debug)]
enum EquipmentType {
    OneHand,
    TwoHand,
    Bow,
    Gun,
    Armor
}

#[derive(Debug)]
struct Equipment {
    name: String,
    prototype: EquipmentType,
    statbonus: Vec<CharacterStat>,
}

fn main() {
    let rusty_sword = Equipment {
        name: "Rusty Sword".to_string(),
        prototype: EquipmentType::OneHand,
        statbonus: vec!(
            CharacterStat::Strength(10)
        ),
    };

    let light_armor = Equipment {
        name: "Light Armor".to_string(),
        prototype: EquipmentType::Armor,
        statbonus: vec!(
            CharacterStat::Defence(10)
        ),
    };

    let mut player = Player::new("Bob");
    
    let raw_input = Input::KeyboardInput(String::from("d"));
    let game_input = Input::check_input(&raw_input);
    println!("Player: {:#?}", player);
    println!("Input: {:#?}", game_input);
    
}
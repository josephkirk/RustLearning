use nalgebra::{Point2, Vector2, Transform2}
struct GameComponent {
    id: i32,
    name: String
}

struct GameObject {
    id: i32,
    name: String,
    components: Vec<GameComponent>
}

struct Point<T> {
    x: T,
    y: T
}

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
    equipment: Vec<String>,
    strait: Vec<CharacterStrait>,
    strength: i32,
    defense: i32,
    luck: i32,
}

#[derive(Debug)]
enum EquipmentType {
    OneHand,
    TwoHand,
    Bow,
    Gun
    Armor
}

#[derive(Debug)]
struct Equipment {
    name: String,
    prototype: EquipmentType,
    statbonus: Vec<CharacterStat>,
}

// struct Transform {
//     matrix: Matrix
// }

// let rusty_sword = Equipment {
//         name: "Rusty Sword",
//         prototype: EquipmentType::OneHand,
//         statbonus: vec!(
//             CharacterStat::Strength(10)
//         ),
//     }

//     let light_armor = Equipment {]

//     let mut player = Player {
//         name: String::from("Bob"),
//         equipment: vec!(rusty_sword, String::from("light armor")),
//         strait: vec!(CharacterStrait::Mad, CharacterStrait::Angry),
//         strength: 10,
//         defense: 10,
//         luck: 5,
//     };
    
//     let raw_input = Input::KeyboardInput(String::from("d"));
//     let game_input = Input::check_input(&raw_input);
//     println!("Player: {:#?}", player);
//     println!("Input: {:#?}", game_input);
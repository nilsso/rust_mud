use tcod::colors;
use tcod::console::{blit, BackgroundFlag, Console, FontLayout, FontType, Offscreen, Root};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const VP_WIDTH: i32 = 50;
const VP_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: colors::Color,
    speed: i32,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: colors::Color, speed: i32) -> Self {
        Object {x, y, char, color, speed}
    }

    pub fn draw(&self, con: &mut Offscreen) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

#[derive(Default, Debug)]
pub struct Input {
    pub move_l: bool,
    pub move_r: bool,
    pub move_u: bool,
    pub move_d: bool,
}

impl Input {
    fn new() -> Self {
        Input {
            ..Default::default()
        }
    }
}

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("SCP Rogue")
        .init();
    tcod::system::set_fps(LIMIT_FPS);

    let mut vp = Offscreen::new(VP_WIDTH, VP_HEIGHT);

    let mut player = Object::new(0, 0, '@', colors::WHITE, 1);

    let mut player_input = Input::new();

    while !root.window_closed() {
        render_client(&mut root, &mut vp, &player);
        let exit = input(&mut root, &mut player, &mut player_input);
        if exit {
            break;
        }
        client_update(&mut root, &mut player, &player_input);
    }
}

pub fn render_client(root: &mut Root, vp: &mut Offscreen, player: &Object) {
    vp.clear();
    player.draw(vp);
    blit(vp, (0, 0), (VP_WIDTH, VP_HEIGHT), root, (0, 0), 1.0, 1.0);
    root.flush();
}

pub fn input(root: &mut Root, object: &mut Object, input: &mut Input) -> bool {
    use tcod::input::{events, Event, Key, Mouse};
    use tcod::input::KeyCode::*;

    for (_flags, event) in events() {
        match event {
            Event::Key(key) => {
                //println!("{:?}", key);
                match key {
                    Key { code: Up, .. } => input.move_u = key.pressed,
                    Key { code: Down, .. } => input.move_d = key.pressed,
                    Key { code: Left, .. } => input.move_l = key.pressed,
                    Key { code: Right, .. } => input.move_r = key.pressed,
                    Key { code: Escape, .. } => return true,
                    _ => {}
                };
            },
            Event::Mouse(mouse) => match mouse {
                _ => {}
            },
        };
    }
    false
}

pub fn client_update(root: &mut Root, object: &mut Object, input: &Input) {
    if input.move_u {
        object.y -= object.speed;
    }
    if input.move_d {
        object.y += object.speed;
    }
    if input.move_l {
        object.x -= object.speed;
    }
    if input.move_r {
        object.x += object.speed;
    }
}

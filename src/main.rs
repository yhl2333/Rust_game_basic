mod map;
mod map_builder;
mod player;
mod camera;
//use map::my_func;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH/2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT/2;
    pub use crate::camera::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
}

use std::{fmt::Error, task::Context};

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        let mb = Self {
            map: map_builder.map.clone(),
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start)
        };

        println!("v's length is {}", map_builder.map.tiles.len());//borrow of moved value: `map_builder.map`

        mb
        
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);//读取键盘的操作，更新玩家和相机
        self.map.render(ctx, &self.camera);//按照相机视角
        self.player.render(ctx, &mut self.camera);
    }
}



fn main() -> BError {
    println!("Hello, world!");
    // let context = BTermBuilder::simple80x50()
    //     .with_title("Dungeon Crawler")
    //     .with_fps_cap(30.0)
    //     .build()?;

    let context=BTermBuilder::new()
            .with_title("Dungeon Crawler")
            .with_fps_cap(30.0)
            .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .with_tile_dimensions(32, 32)
            .with_resource_path("resources/")
            .with_font("dungeonfont.png", 32, 32)
            .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
            .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
            .build()?;

    main_loop(context, State::new())
}

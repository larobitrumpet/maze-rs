use std::rc::Rc;
use std::borrow::Borrow;
use std::process::exit;

extern crate allegro;
extern crate allegro_image;
use allegro::*;
use allegro_image::*;

use maze_rs::maze::Maze;
use maze_rs::point::Point;
use maze_rs::random::Random;

fn disp_post_draw(core: &Core, display: &mut Display, buffer: &Bitmap, buffer_w: usize, buffer_h: usize) {
    let buffer_aspect: f32 = buffer_w as f32 / buffer_h as f32;

    let display_w: usize = display.get_width().try_into().unwrap();
    let display_h: usize = display.get_height().try_into().unwrap();
    let display_aspect: f32 = display_w as f32 / display_h as f32;

    let display_scale_w: f32 = display_w as f32 / buffer_w as f32;
    let display_scale_h: f32 = display_h as f32 / buffer_h as f32;

    let (disp_w, disp_h, disp_w_offset, disp_h_offset) = if display_aspect < buffer_aspect {
        let disp_w: f32 = buffer_w as f32 * display_scale_w;
        let disp_h: f32 = buffer_h as f32 * display_scale_w;

        let dd_h: f32 = buffer_h as f32 * display_scale_h;

        let disp_h_offset: f32 = (dd_h - disp_h) / 2.0;
        let disp_w_offset: f32 = 0.0;

        (disp_w, disp_h, disp_w_offset, disp_h_offset)
    } else if display_aspect > buffer_aspect {
        let disp_w: f32 = buffer_w as f32 * display_scale_h;
        let disp_h: f32 = buffer_h as f32 * display_scale_h;

        let dd_w: f32 = buffer_w as f32 * display_scale_w;

        let disp_w_offset: f32 = (dd_w - disp_w) / 2.0;
        let disp_h_offset: f32 = 0.0;

        (disp_w, disp_h, disp_w_offset, disp_h_offset)
    } else {
        let disp_w: f32 = buffer_w as f32 * display_scale_w;
        let disp_h: f32 = buffer_h as f32 * display_scale_h;

        (disp_w, disp_h, 0.0, 0.0)
    };

    core.set_target_bitmap(Some(display.get_backbuffer()));
    core.clear_to_color(Color::from_rgb(0, 0, 0));
    core.draw_scaled_bitmap(buffer, 0.0, 0.0, buffer_w as f32, buffer_h as f32, disp_w_offset, disp_h_offset, disp_w, disp_h, Flag::zero());

    core.flip_display();
}

fn draw(core: &Core, display: &mut Display, buffer: &Bitmap, buffer_w: usize, buffer_h: usize, sprites: &Sprites, maze: &Maze) {
    core.set_target_bitmap(Some(buffer));
    draw_maze(core, display, buffer, buffer_w, buffer_h, sprites, maze);
    disp_post_draw(core, display, buffer, buffer_w, buffer_h);
}

fn draw_maze(core: &Core, display: &mut Display, buffer: &Bitmap, buffer_w: usize, buffer_h: usize, sprites: &Sprites, maze: &Maze) -> () {
    core.set_target_bitmap(Some(buffer));
    for x in 0..maze.width() {
        for y in 0..maze.height() {
            let p = Point::new(x, y);
            core.draw_bitmap(sprites.get_tile(maze, p), (p.x() * sprites.tile_w()) as f32, (p.y() * sprites.tile_h()) as f32, Flag::zero());
        }
    }
    disp_post_draw(core, display, buffer, buffer_w, buffer_h);
}

fn update_maze_display(core: &Core, display: &mut Display, buffer: &Bitmap, buffer_w: usize, buffer_h: usize, queue: &EventQueue, sprites: &Sprites, maze: &mut Maze) -> () {
        if !queue.is_empty() {
            match queue.wait_for_event() {
                DisplayClose{..} => exit(0),
                KeyDown{keycode: k, ..} if k == KeyCode::Escape => exit(0),
                DisplayResize{..} => display.acknowledge_resize().unwrap(),
                _ => (),
            }
        }

    core.set_target_bitmap(Some(buffer));
    while let Some(p) = maze.update_pop() {
        core.draw_bitmap(sprites.get_tile(maze, p), (p.x() * sprites.tile_w()) as f32, (p.y() * sprites.tile_h()) as f32, Flag::zero());
    }
    disp_post_draw(core, display, buffer, buffer_w, buffer_h);
}

pub struct Sprites {
    tiles: Vec<Vec<Bitmap>>,
    tile_w: usize,
    tile_h: usize,
}

impl Sprites {
    pub fn init(core: &Core) -> Sprites {
        let tile_w = 8;
        let tile_h = 8;
        let sheet = Bitmap::load(core, "./data/maze_tiles.png").unwrap();
        let mut tiles: Vec<Vec<Bitmap>> = vec![];
        for i in 0..3 {
            let mut temp: Vec<Bitmap> = vec![];
            for j in 0..16 {
                temp.push(Self::sprite_grab(&sheet, i * tile_w as i32, j * tile_h as i32, tile_w as i32, tile_h as i32));
            }
            tiles.push(temp);
        }

        Sprites {tiles, tile_w, tile_h}
    }

    fn sprite_grab(sheet: &Bitmap, x: i32, y: i32, w: i32, h: i32) -> Bitmap {
        let bit: Rc<SubBitmap> = sheet.create_sub_bitmap(x, y, w, h).unwrap().upgrade().unwrap();
        let bit: &SubBitmap = bit.borrow();
        bit.to_bitmap().unwrap()
    }

    pub fn get_tile(&self, maze: &Maze, p: Point) -> &Bitmap {
        &self.tiles[maze.cell_type(p) as usize][maze.cell_passage(p) as usize]
    }

    pub fn tile_w(&self) -> usize {
        self.tile_w
    }

    pub fn tile_h(&self) -> usize {
        self.tile_h
    }
}

allegro_main! {
    let core = Core::init().unwrap();
    let _image = ImageAddon::init(&core).unwrap();

    let sprites = Sprites::init(&core);
    let (width, height) = maze_rs::user_input::get_dimentions();
    let buffer_w: usize = width * sprites.tile_w();
    let buffer_h: usize = height * sprites.tile_h();
    let mut rand = Random::new();

    let algorithm = maze_rs::user_input::get_algorithm();
    let wall_adder = algorithm == 4;
    let algorithm = match algorithm {
        0 => maze_rs::algorithms::recursive_backtracking,
        1 => maze_rs::algorithms::eller,
        2 => maze_rs::algorithms::kruskal,
        3 => maze_rs::algorithms::prim,
        4 => maze_rs::algorithms::recursive_division,
        5 => maze_rs::algorithms::aldous_broder,
        6 => maze_rs::algorithms::wilson,
        _ => panic!("This should be unreachable."),
    };

    let mut maze = Maze::new(width, height, wall_adder);

    let mut display = Display::new(&core, 800, 600).unwrap();
    let buffer = Bitmap::new(&core, buffer_w as i32, buffer_h as i32).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

    core.install_keyboard().unwrap();

    //core.set_new_display_flags(FULLSCREEN_WINDOW);
    core.set_new_display_flags(RESIZABLE);
    core.set_new_display_option(DisplayOption::SampleBuffers, 1, DisplayOptionImportance::Suggest);
    core.set_new_display_option(DisplayOption::Samples, 8, DisplayOptionImportance::Suggest);
    core.set_new_display_option(DisplayOption::DepthSize, 16, DisplayOptionImportance::Suggest);

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());

    draw_maze(&core, &mut display, &buffer, buffer_w, buffer_h, &sprites, &maze);

    algorithm(
        &mut maze, &mut rand,
        &mut |maze: &mut Maze| {
            update_maze_display(&core, &mut display, &buffer, buffer_w, buffer_h, &queue, &sprites, maze);
        }
    );

    let mut redraw = true;
    timer.start();
    'exit: loop {
        if redraw && queue.is_empty() {
            draw(&core, &mut display, &buffer, buffer_w, buffer_h, &sprites, &maze);
            redraw = false;
        }

        match queue.wait_for_event() {
            DisplayClose{..} => break 'exit,
            KeyDown{keycode: k, ..} if k == KeyCode::Escape => break 'exit,
            DisplayResize{..} => display.acknowledge_resize().unwrap(),
            TimerTick{..} => redraw = true,
            _ => (),
        }
    }
}

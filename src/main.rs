

use std::{env, path};

use ggez::conf::FullscreenType;
use ggez::{context, event, GameError};
use ggez::graphics::{self, Color, Mesh, PxScale, Rect, TextFragment};
use ggez::{Context, GameResult};
use ggez::glam::*;

struct MainState {
    board: arvidkr_chess::Board
}

fn draw_board(canvas: &mut graphics::Canvas, ctx: &Context) -> Result<(), GameError>{
    Ok(for i in 0..8{
        for j in 0..8{
            let mut color = Color::from_rgb(118,150,86);
            if (i + j) % 2 == 0{
                color = Color::WHITE;
            }
            let bounds = Rect::new(0.0, 0.0, 90.0, 90.0);
            let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds , color)?;
            canvas.draw(&rectangle, Vec2::new((j*90) as f32, (i*90) as f32));
        }
    })
}
 #[derive(PartialEq)]
 enum Direction {
     Vertical,
     Horizontal
 }
fn draw_repeated_elements(canvas: &mut graphics::Canvas, elements: [&str; 8], direction: Direction){
    for i in 0..8{
        let mut color = Color::from_rgb(118,150,86);
        if i % 2 == 0{
            color = Color::WHITE;
        }
        let mut drawParam = ggez::glam::Vec2::new((i*90+60+10) as f32, 688.0);
        if direction == Direction::Vertical{
            drawParam = ggez::glam::Vec2::new(3.5, (630-i*90) as f32);
        }
        canvas.draw(
            &graphics::Text::new(TextFragment{
                text: elements[i].into(),
                font: Some("LiberationMono".into()),
                color: Some(color),
                scale: Some(PxScale::from(32.0)),
            }),
            drawParam);
    }
}
fn draw_board_indexing(canvas: &mut graphics::Canvas, ctx: &Context) -> Result<(), GameError>{
    Ok({
        draw_repeated_elements(canvas, ["a","b","c","d","e","f","g","h"], Direction::Horizontal);
        draw_repeated_elements(canvas, ["8","7","6","5","4","3","2","1"], Direction::Vertical);
       }
      )
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf")?,
        );
        let s = MainState { board: arvidkr_chess::Board::new(), };
        return Ok(s);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );
        draw_board(&mut canvas, &ctx);
        draw_board_indexing(&mut canvas, &ctx);

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let window_mode = ggez::conf::WindowMode{
        width: 720.0,
        height: 720.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        transparent: false,
        borderless: false,
        min_height: 720.0,
        min_width: 720.0,
        max_height: 720.0,
        max_width: 720.0,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false,
        logical_size: None};
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let cb = ggez::ContextBuilder::new("chess", "caspervk").window_mode(window_mode).add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
    
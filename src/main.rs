

use std::{env, path};

use draw::{draw_board_indexing, draw_board_pieces, draw_board_rectangles};
use ggez::conf::FullscreenType;
use ggez::{context, event, GameError};
use ggez::graphics::{self, Color, DrawParam, Drawable, Mesh, PxScale, Rect, TextFragment};
use ggez::{Context, GameResult};
use ggez::glam::*;

mod draw;
struct PieceImages{
    black_rook: graphics::Image,
    black_knight: graphics::Image,
    black_bishop: graphics::Image,
    black_king: graphics::Image,
    black_queen: graphics::Image,
    black_pawn: graphics::Image,

    white_rook: graphics::Image,
    white_knight: graphics::Image,
    white_bishop: graphics::Image,
    white_king: graphics::Image,
    white_queen: graphics::Image,
    white_pawn: graphics::Image,
}
struct MainState {
    board: chess_lib::Game,
    piece_images: PieceImages
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf")?,
        );
        let mut s = MainState { 
            board: chess_lib::Game::new(), 
            piece_images: PieceImages{
            black_rook: graphics::Image::from_path(ctx, "/r_black.png")?,
            black_knight: graphics::Image::from_path(ctx, "/n_black.png")?,
            black_bishop: graphics::Image::from_path(ctx, "/b_black.png")?,
            black_king: graphics::Image::from_path(ctx, "/k_black.png")?,
            black_queen: graphics::Image::from_path(ctx, "/q_black.png")?,
            black_pawn: graphics::Image::from_path(ctx, "/p_black.png")?,

            white_rook: graphics::Image::from_path(ctx, "/r_white.png")?,
            white_knight: graphics::Image::from_path(ctx, "/n_white.png")?,
            white_bishop: graphics::Image::from_path(ctx, "/b_white.png")?,
            white_king: graphics::Image::from_path(ctx, "/k_white.png")?,
            white_queen: graphics::Image::from_path(ctx, "/q_white.png")?,
            white_pawn: graphics::Image::from_path(ctx, "/p_white.png")?,
            }
        
        };
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
        draw_board_rectangles(&mut canvas, &ctx);
        draw_board_indexing(&mut canvas, &ctx);
        draw_board_pieces(&mut canvas, self.board.get_board(), &self.piece_images);


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
    


use std::time::Instant;
use std::{env, path};

use chess_lib::{GameState, Move, Position};
use draw::{draw_board_indexing, draw_board_pieces, draw_board_rectangles, draw_highlighted_squares};
use ggez::conf::FullscreenType;
use ggez::event::MouseButton;
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
    piece_images: PieceImages,
    mouse_down: bool,
    mouse_down_x: f32,
    mouse_down_y: f32,
    highlighted_movements: Option<Vec<u32>>,
    selected_piece_index: Option<usize>,
    last_click_time: Instant,
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
            },
            mouse_down: false,
            mouse_down_x: 0.,
            mouse_down_y: 0.,
            highlighted_movements: None,
            selected_piece_index: None,
            last_click_time: Instant::now(),
        
        };
        return Ok(s);
    }
}

fn get_pos_index(x: f32, y: f32) -> usize{
    return ((x) / 90.).floor() as usize + (y / 90.).floor() as usize * 8;
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_down_x = x;
        self.mouse_down_y = y;
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_down = false;
        if(x == self.mouse_down_x && y == self.mouse_down_y && self.last_click_time.elapsed().as_millis() > 50){ // click
            self.last_click_time = Instant::now();
            if(self.highlighted_movements.is_none()){
                let index = get_pos_index(x, y);
                let selectedPiece = self.board.get_board()[index];
                if(!selectedPiece.is_none() && selectedPiece.unwrap().colour == self.board.get_active_colour()){
                    let mut positions = Vec::new();
                    for square in self.board.get_legal_moves_from(chess_lib::Position::new_from_idx(index).unwrap()).as_ref().unwrap().iter(){
                        positions.push(square.to.idx as u32);
                    }
                    self.highlighted_movements = Some(positions);
                    self.selected_piece_index = Some(index);
                }
            }
            else{
                let index = get_pos_index(x, y);
                if(self.highlighted_movements.as_ref().unwrap().iter().any(|&a| a == index as u32)){
                    self.board.make_move(Move::new(&self.board, Position::new_from_idx(self.selected_piece_index.unwrap()).unwrap(), Position::new_from_idx(index).unwrap()).unwrap());
                }
                self.highlighted_movements = None;
                self.selected_piece_index = None;
            }
        }
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
        if(!self.highlighted_movements.is_none()){
            draw_highlighted_squares(&mut canvas, &ctx, self.highlighted_movements.as_ref().unwrap());
        }
        if(self.board.get_game_state() == GameState::GameOver){
        }


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
    
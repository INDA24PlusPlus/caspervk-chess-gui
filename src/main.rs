

use std::net::TcpStream;
use std::time::Instant;
use std::{env, path};

use chess_lib::{Colour, GameOverReason, GameState, Move, Position};
use draw::{draw_board_indexing, draw_board_pieces, draw_board_rectangles, draw_game_over_window, draw_highlighted_squares, draw_promotion_selection_window, draw_request_draw_button};
use ggez::conf::FullscreenType;
use ggez::event::MouseButton;
use ggez::mint::Point2;
use ggez::{context, event, GameError};
use ggez::graphics::{self, Color, DrawParam, Drawable, Mesh, PxScale, Rect, TextFragment};
use ggez::{Context, GameResult};
use ggez::glam::*;
use network::{await_move, chess_lib_state_to_network_state, do_move, send_ack, start_client, start_server};

mod draw;
mod network;
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
    stream: TcpStream,
    piece_images: PieceImages,

    player_side: chess_lib::Colour,
    opponent_name: Option<String>,

    mouse_down: bool,
    mouse_down_x: f32,
    mouse_down_y: f32,
    last_click_time: Instant,

    highlighted_movements: Option<Vec<u32>>,
    selected_piece_index: Option<usize>,
    awaiting_promotion_choice: bool,
    move_to_make_after_promotion: Option<chess_lib::Move>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let args: Vec<_> = env::args().collect();
        let mut stream;
        let mut opponent_name;
        let mut player_side = chess_lib::Colour::White; // Who doesnt want to start white:)
        println!(" {} {} ", args[2], args[3]);
        let mut board = chess_lib::Game::new();
        if(args.len() < 3){
            return GameResult::Err((GameError::CustomError("Invalid command line arguments.".to_string())));
        }
        if(args[1] == "s"){
            (stream, opponent_name) = start_server(&args[2], &args[3]);
        }
        else if(args[1] == "c"){
            (stream, opponent_name, player_side) = start_client(&args[2], &args[3]);
            let _Move = await_move(&mut stream);
            board.make_move(Move::new(&board, Position::new(_Move.from.0 as usize, _Move.from.1 as usize).unwrap(), Position::new(_Move.to.0 as usize, _Move.to.1 as usize).unwrap()).unwrap());
            send_ack(&mut stream, true, None);
        }
        else{
            return GameResult::Err((GameError::CustomError("Invalid command line arguments. Please select if server or client with s or c as first argument".to_string())));
        }

        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf")?,
        );
        let mut s = MainState { 
            board: board, 
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
            awaiting_promotion_choice: false,        
            move_to_make_after_promotion: None,
            opponent_name: opponent_name,
            stream: stream,
            player_side: player_side
        };
        return Ok(s);
    }
}

fn get_pos_index(x: f32, y: f32) -> usize{
    return ((x) / 90.).floor() as usize + (y / 90.).floor() as usize * 8;
}

fn get_selected_promotion(x: f32, y: f32) -> Option<chess_lib::PieceType>{
    if(Rect::new(255., 295., 55., 55.).contains(Point2{x, y})){
        return Some(chess_lib::PieceType::Queen);
    }
    if(Rect::new(315., 295., 55., 55.).contains(Point2{x, y})){
        return Some(chess_lib::PieceType::Knight);
    }
    if(Rect::new(375., 295., 55., 55.).contains(Point2{x, y})){
        return Some(chess_lib::PieceType::Rook);
    }
    if(Rect::new(435., 295., 55., 55.).contains(Point2{x, y})){
        return Some(chess_lib::PieceType::Bishop);
    }
    return None;
}

impl MainState{
    fn is_click(&mut self, x: f32, y: f32) -> bool{
        if(x == self.mouse_down_x && y == self.mouse_down_y && !self.last_click_time.elapsed().as_millis() < 50){
            return false;
        }
        self.last_click_time = Instant::now();
        return true;
    }
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
        if self.is_click(x, y){ // click
            if(self.awaiting_promotion_choice){
                let mut promotion_choice = get_selected_promotion(x, y);
                if(!promotion_choice.is_none()){
                    let mut mv = self.move_to_make_after_promotion.unwrap();
                    mv.promotion_choice = promotion_choice;
                    let (ok, gamestate) = do_move(&mut self.stream, mv, None);
                    if(self.player_side == chess_lib::Colour::White || ok){
                        self.board.make_move(mv);
                        let _Move = await_move(&mut self.stream); // opponent move
                        let result = self.board.make_move(Move::new(&self.board, Position::new(_Move.from.0 as usize, _Move.from.1 as usize).unwrap(), Position::new(_Move.to.0 as usize, _Move.to.1 as usize).unwrap()).unwrap());
                        send_ack(&mut self.stream, result.is_err(), chess_lib_state_to_network_state(self.board.get_game_state()));
                    }
                    self.move_to_make_after_promotion = None;
                    self.awaiting_promotion_choice = false;
                }
            }
            else if(self.board.get_active_colour() == self.player_side){
                let index = get_pos_index(x, y);
                if(Rect::new(743., 306., 141., 52.).contains(Point2{x, y})){
                    self.board.submit_draw();
                }
                if(x > 700.){
                    return Ok(());
                }
                if(self.highlighted_movements.is_none()){
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
                    if(self.highlighted_movements.as_ref().unwrap().iter().any(|&a| a == index as u32)){
                        let mv = Move::new(&self.board, Position::new_from_idx(self.selected_piece_index.unwrap()).unwrap(), Position::new_from_idx(index).unwrap()).unwrap();
                        if(mv.is_promotion()){
                            self.awaiting_promotion_choice = true;
                            self.move_to_make_after_promotion = Some(mv);
                        }
                        else{
                            let (ok, gamestate) = do_move(&mut self.stream, mv, None);
                            if(self.player_side == chess_lib::Colour::White || ok){
                                self.board.make_move(Move::new(&self.board, Position::new_from_idx(self.selected_piece_index.unwrap()).unwrap(), Position::new_from_idx(index).unwrap()).unwrap());
                                let _Move = await_move(&mut self.stream); // opponent move
                                let result = self.board.make_move(Move::new(&self.board, Position::new(_Move.from.0 as usize, _Move.from.1 as usize).unwrap(), Position::new(_Move.to.0 as usize, _Move.to.1 as usize).unwrap()).unwrap());
                                send_ack(&mut self.stream, result.is_err(), chess_lib_state_to_network_state(self.board.get_game_state()));
                            }
                        }
                    }
                    self.highlighted_movements = None;
                    self.selected_piece_index = None;
                }
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
            draw_game_over_window(&mut canvas, ctx,  self.board.get_game_over_reason().unwrap(), self.board.get_active_colour()); 
        }
        if(self.awaiting_promotion_choice){ 
            draw_promotion_selection_window(&mut canvas, ctx, self.board.get_active_colour(), &self.piece_images); 
        }
        draw_request_draw_button(&mut canvas, ctx);

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let window_mode = ggez::conf::WindowMode{
        width: 900.0,
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
    
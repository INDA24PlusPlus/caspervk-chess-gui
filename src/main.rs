

use std::{env, path};

use ggez::conf::FullscreenType;
use ggez::{context, event, GameError};
use ggez::graphics::{self, Color, DrawParam, Drawable, Mesh, PxScale, Rect, TextFragment};
use ggez::{Context, GameResult};
use ggez::glam::*;
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

fn draw_board_rectangles(canvas: &mut graphics::Canvas, ctx: &Context) -> Result<(), GameError>{
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

fn draw_board_pieces(canvas: &mut graphics::Canvas, board: [Option<chess_lib::Piece>; 8*8], piece_images: &PieceImages){
    for i in 0..8{
        for j in 0..8{
            if(board[i*8+j].is_some()){
                let piece = board[i*8+j].unwrap();
                let drawParam = vec2((j*90+13) as f32, (i*90+13) as f32);
                let image_to_draw = match piece.to_char() {
                    'R' => {
                        match piece.colour{
                            chess_lib::Colour::Black => &piece_images.black_rook,
                            chess_lib::Colour::White => &piece_images.white_rook,
                        }
                    },
                    'N' =>                         
                        match piece.colour{
                            chess_lib::Colour::Black => &piece_images.black_knight,
                            chess_lib::Colour::White => &&piece_images.white_knight,
                        },
                    'B' => {
                        match piece.colour{
                            chess_lib::Colour::Black => &piece_images.black_bishop,
                            chess_lib::Colour::White => &piece_images.white_bishop,
                        }
                    },
                    'K' => {
                        match piece.colour{
                            chess_lib::Colour::Black => &piece_images.black_king,
                            chess_lib::Colour::White => &piece_images.white_king,
                        }
                    },
                    'Q' => {
                        match piece.colour{
                            chess_lib::Colour::Black => &piece_images.black_queen,
                            chess_lib::Colour::White => &piece_images.white_queen,
                        }
                    },
                    'P' => {
                        match piece.colour{
                            chess_lib::Colour::Black => &piece_images.black_pawn,
                            chess_lib::Colour::White => &piece_images.white_pawn,
                        }
                    },
                    _ => {panic!();},
                };
                canvas.draw(image_to_draw, drawParam);
            }
        }
    }
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
    
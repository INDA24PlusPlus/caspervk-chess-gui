use std::vec;

use chess_lib::GameOverReason;
use ggez::{glam::{vec2, Vec2}, graphics::{self, Color, PxScale, Rect, TextFragment}, Context, GameError};

use crate::PieceImages;

#[derive(PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal
}
pub fn draw_repeated_elements(canvas: &mut graphics::Canvas, elements: [&str; 8], direction: Direction){
   for i in 0..8{
       let mut color = Color::from_rgb(118,150,86);
       if i % 2 == 0{
           color = Color::WHITE;
       }
       let mut drawParam = ggez::glam::Vec2::new((i*90+70) as f32, 690.0);
       if direction == Direction::Vertical{
           drawParam = ggez::glam::Vec2::new(2., (632-i*90) as f32);
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
pub fn draw_board_indexing(canvas: &mut graphics::Canvas, ctx: &Context) -> Result<(), GameError>{
   Ok({
       draw_repeated_elements(canvas, ["a","b","c","d","e","f","g","h"], Direction::Horizontal);
       draw_repeated_elements(canvas, ["8","7","6","5","4","3","2","1"], Direction::Vertical);
      }
     )
}

pub fn draw_board_pieces(canvas: &mut graphics::Canvas, board: [Option<chess_lib::Piece>; 8*8], piece_images: &PieceImages){
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

pub fn draw_board_rectangles(canvas: &mut graphics::Canvas, ctx: &Context) -> Result<(), GameError>{
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

//returns x,y
fn get_index_pos(index: u32) -> (u32, u32){
    return ((index as u32 % 8)*90+45, index/8*90+45);
}

pub fn draw_highlighted_squares(canvas: &mut graphics::Canvas, ctx: &Context, to_draw: &Vec<u32>) -> Result<(), GameError>{
    Ok((
        for index in to_draw{
            let (x, y) = get_index_pos(*index);
            let circle = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), Vec2::new(0., 0.), 15., 0.2, Color::RED)?;
            canvas.draw(&circle, Vec2::new(x as f32, y as f32))
        }
    ))
}

pub fn draw_game_over_window(canvas: &mut graphics::Canvas, ctx: &Context, game_over_reason: GameOverReason, color: chess_lib::Colour){
    let text = match game_over_reason{
        GameOverReason::Checkmate => {
            match color {
                chess_lib::Colour::White => "White won by checkmate!",
                chess_lib::Colour::Black => "Black won by checkmate!"
            }
        },
        GameOverReason::FivefoldRepetitionRule => "Draw by five fold repetition",
        GameOverReason::SeventyFiveMoveRule => "Draw by seventy five move rule",
        GameOverReason::Stalemate => "Drawby stale mate",
        _ => {panic!()}
    };
    let bounds = Rect::new(0.0, 0.0, 300.0, 130.0);
    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds , Color::RED).unwrap();
    canvas.draw(&rectangle, Vec2::new(220., 260.));
    canvas.draw(
        &graphics::Text::new(TextFragment{
            text: String::from(text),
            font: Some("LiberationMono".into()),
            color: Some(Color::BLACK),
            scale: Some(PxScale::from(32.0)),
        }), Vec2::new(295., 310.)
);
}

pub fn draw_promotion_selection_window(canvas: &mut graphics::Canvas, ctx: &Context, color: chess_lib::Colour, piece_images: &PieceImages){
    let bounds = Rect::new(0.0, 0.0, 300.0, 130.0);
    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds , Color::RED).unwrap();
    canvas.draw(&rectangle, Vec2::new(220., 260.));
    if(color.is_black()){
        canvas.draw(&piece_images.black_queen, ggez::glam::Vec2::new(255 as f32, 295.0));
        canvas.draw(&piece_images.black_knight, ggez::glam::Vec2::new(315 as f32, 295.0));
        canvas.draw(&piece_images.black_rook, ggez::glam::Vec2::new(375 as f32, 295.0));
        canvas.draw(&piece_images.black_bishop, ggez::glam::Vec2::new(435 as f32, 295.0));
    }
    else{
        canvas.draw(&piece_images.white_queen, ggez::glam::Vec2::new(255 as f32, 295.0));
        canvas.draw(&piece_images.white_knight, ggez::glam::Vec2::new(315 as f32, 295.0));
        canvas.draw(&piece_images.white_rook, ggez::glam::Vec2::new(375 as f32, 295.0));
        canvas.draw(&piece_images.white_bishop, ggez::glam::Vec2::new(435 as f32, 295.0));
    }
}

pub fn draw_request_draw_button(canvas: &mut graphics::Canvas, ctx: &Context){
    let bounds = Rect::new(0.0, 0.0, 140.0, 50.0);
    let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds , Color::RED).unwrap();
    canvas.draw(&rectangle, Vec2::new(745., 310.));
    canvas.draw(
        &graphics::Text::new(TextFragment{
            text: "Request draw".to_string(),
            font: Some("LiberationMono".into()),
            color: Some(Color::BLACK),
            scale: Some(PxScale::from(20.0)),
        }), Vec2::new(750., 328.));
}
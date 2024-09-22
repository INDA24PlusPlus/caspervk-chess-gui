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
       let mut drawParam = ggez::glam::Vec2::new((i*90+60+10) as f32, 690.0);
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
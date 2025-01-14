use crate::*;

pub struct Assets {
    pub graphics: HashMap<String, Image>
}

impl Assets {
    pub fn load_all(ctx: &mut Context) -> Self {
        let mut graphics: HashMap<String, Image> = HashMap::new();
        graphics.insert("board".into(), Image::from_path(ctx, "/Board/Board.png").unwrap());

        graphics.insert("whiteselect".into(), Image::from_path(ctx, "/Selected_White.png").unwrap());
        graphics.insert("blackselect".into(), Image::from_path(ctx, "/Selected_Black.png").unwrap());

        graphics.insert("whitehighlight".into(), Image::from_path(ctx, "/Highlight_White.png").unwrap());
        graphics.insert("blackhighlight".into(), Image::from_path(ctx, "/Highlight_Black.png").unwrap());

        graphics.insert("movehighlight".into(), Image::from_path(ctx, "/Move_Highlight.png").unwrap());
        graphics.insert("capturehighlight".into(), Image::from_path(ctx, "/Capture_Highlight.png").unwrap());
        graphics.insert("checkhighlight".into(), Image::from_path(ctx, "/Check_Highlight.png").unwrap());


        graphics.insert("blackpawn".into(), Image::from_path(ctx, "/Pieces/BlackPieces/Pawn.png").unwrap());
        graphics.insert("blackknight".into(), Image::from_path(ctx, "/Pieces/BlackPieces/Knight.png").unwrap());
        graphics.insert("blackbishop".into(), Image::from_path(ctx, "/Pieces/BlackPieces/Bishop.png").unwrap());
        graphics.insert("blackrook".into(), Image::from_path(ctx, "/Pieces/BlackPieces/Rook.png").unwrap());
        graphics.insert("blackqueen".into(), Image::from_path(ctx, "/Pieces/BlackPieces/Queen.png").unwrap());
        graphics.insert("blackking".into(), Image::from_path(ctx, "/Pieces/BlackPieces/King.png").unwrap());

        graphics.insert("whitepawn".into(), Image::from_path(ctx, "/Pieces/WhitePieces/Pawn.png").unwrap());
        graphics.insert("whiteknight".into(), Image::from_path(ctx, "/Pieces/WhitePieces/Knight.png").unwrap());
        graphics.insert("whitebishop".into(), Image::from_path(ctx, "/Pieces/WhitePieces/Bishop.png").unwrap());
        graphics.insert("whiterook".into(), Image::from_path(ctx, "/Pieces/WhitePieces/Rook.png").unwrap());
        graphics.insert("whitequeen".into(), Image::from_path(ctx, "/Pieces/WhitePieces/Queen.png").unwrap());
        graphics.insert("whiteking".into(), Image::from_path(ctx, "/Pieces/WhitePieces/King.png").unwrap());



        Self {
            graphics
        }
    } 
}
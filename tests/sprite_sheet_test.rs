use spriter::io::sprite_sheet::SpriteSheet;

#[test]
fn test_grid_dimensions() {
    let sheet = SpriteSheet::new(256, 128, 32, 32);
    assert_eq!(sheet.columns, 8);
    assert_eq!(sheet.rows, 4);
    assert_eq!(sheet.total_frames(), 32);
}

#[test]
fn test_frame_rect() {
    let sheet = SpriteSheet::new(128, 64, 32, 32);
    let (x, y) = sheet.frame_origin(0);
    assert_eq!((x, y), (0, 0));

    let (x, y) = sheet.frame_origin(3);
    assert_eq!((x, y), (96, 0));

    let (x, y) = sheet.frame_origin(4);
    assert_eq!((x, y), (0, 32));
}

#[test]
fn test_non_divisible_dimensions_reports_remainder() {
    let sheet = SpriteSheet::new(100, 50, 32, 32);
    assert_eq!(sheet.columns, 3);
    assert_eq!(sheet.rows, 1);
    assert_eq!(sheet.remainder_x(), 4);
    assert_eq!(sheet.remainder_y(), 18);
}

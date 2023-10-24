use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    // INSTEAD of rendering the whole map... We generate the map (map builder), then
    // get the camera position, which is based on the player position, and then we
    // render only the tiles that are inside the camera view.
    // KEY CONCEPT

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            if map.inside_map(Point::new(x, y)) {
                let idx = map_idx(x, y);
                let (glyph, color) = match map.tiles[idx] {
                    TyleType::Floor => (to_cp437('.'), WHITE),
                    TyleType::Wall => (to_cp437('#'), CRIMSON),
                };

                draw_batch.set(pt - offset, ColorPair::new(color, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}

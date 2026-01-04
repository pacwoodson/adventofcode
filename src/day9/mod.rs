use crate::utils::lines_to_vec;

#[derive(Debug, Clone, Copy, Default)]
struct Tile {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, Default)]
struct TilesArea {
    tile_a: Tile,
    tile_b: Tile,
    area: u64,
}

fn has_opposites(tiles_area: &TilesArea, tiles: &Vec<Tile>) -> bool {
    let tile_c = Tile {
        x: tiles_area.tile_a.x,
        y: tiles_area.tile_b.y,
    };
    let tile_d = Tile {
        x: tiles_area.tile_b.x,
        y: tiles_area.tile_a.y,
    };

    if tiles_area.tile_a.x > tiles_area.tile_b.x {
        // b is left
        if tiles_area.tile_a.y > tiles_area.tile_b.y {
            // b is left top, a is right bottom
            // c is right top, d is left bottom
            let c_is_valid = tiles
                .iter()
                .find(|tile| tile_c.x <= tile.x && tile_c.y >= tile.y);
            let d_is_valid = tiles
                .iter()
                .find(|tile| tile_d.x >= tile.x && tile_d.y <= tile.y);

            if c_is_valid.is_some() && d_is_valid.is_some() {
                return true;
            }
        } else {
            // a is top rigth, b is left bottom
            // c is right bottom , d is left top
            let c_is_valid = tiles
                .iter()
                .find(|tile| tile_c.x <= tile.x && tile_c.y <= tile.y);
            let d_is_valid = tiles
                .iter()
                .find(|tile| tile_d.x >= tile.x && tile_d.y >= tile.y);

            if c_is_valid.is_some() && d_is_valid.is_some() {
                return true;
            }
        }
    } else {
        // a is left
        if tiles_area.tile_a.y > tiles_area.tile_b.y {
            // b is rigt top, a is left bottom
            // c is left top, d is right bottom
            let c_is_valid = tiles
                .iter()
                .find(|tile| tile_c.x >= tile.x && tile_c.y >= tile.y);
            let d_is_valid = tiles
                .iter()
                .find(|tile| tile_d.x <= tile.x && tile_d.y <= tile.y);

            if c_is_valid.is_some() && d_is_valid.is_some() {
                return true;
            }
        } else {
            // a is left top, b is bottom right
            // c is left bottom, d is right top
            let c_is_valid = tiles
                .iter()
                .find(|tile| tile_c.x >= tile.x && tile_c.y <= tile.y);
            let d_is_valid = tiles
                .iter()
                .find(|tile| tile_d.x <= tile.x && tile_d.y >= tile.y);

            if c_is_valid.is_some() && d_is_valid.is_some() {
                return true;
            }
        }
    }
    return false;
}

pub fn solve(input: &String, _: bool) -> (String, String) {
    let tiles: Vec<Tile> = lines_to_vec(input)
        .iter()
        .map(|line| {
            let positions: Vec<i64> = line.split(',').map(|p| p.parse().unwrap()).collect();
            Tile {
                x: positions[0],
                y: positions[1],
            }
        })
        .collect();

    let mut tiles_areas: Vec<TilesArea> = vec![];
    for (i_a, tile_a) in tiles.iter().enumerate() {
        for (i_b, tile_b) in tiles.iter().enumerate() {
            if i_a <= i_b {
                continue;
            }
            let area =
                (((tile_a.x - tile_b.x).abs() + 1) * ((tile_a.y - tile_b.y).abs() + 1)) as u64;
            let tiles_area = TilesArea {
                tile_a: *tile_a,
                tile_b: *tile_b,
                area,
            };
            tiles_areas.push(tiles_area);
        }
    }
    tiles_areas.sort_by(|a, b| b.area.cmp(&a.area));
    let max_area = tiles_areas[0].area;

    // dbg!(tiles_areas[0], tiles_areas[1], "tiles_areas");

    for (_, tiles_area) in tiles_areas.iter().enumerate() {
        if has_opposites(&tiles_area, &tiles) {
            let mut good = true;
            for tile in tiles.iter() {
                if tile.x > tiles_area.tile_a.x.min(tiles_area.tile_b.x)
                    && tile.x < tiles_area.tile_a.x.max(tiles_area.tile_b.x)
                    && tile.y > tiles_area.tile_a.y.min(tiles_area.tile_b.y)
                    && tile.y < tiles_area.tile_a.y.max(tiles_area.tile_b.y)
                {
                    good = false;
                    break;
                }
            }

            if good {
                dbg!(tiles_area);
                return (max_area.to_string(), tiles_area.area.to_string());
            }
        }
    }
    panic!("not found");
}

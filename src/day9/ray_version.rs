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

fn point_in_polygon(point: &Tile, polygon: &Vec<Tile>) -> bool {
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let p1 = &polygon[i];
        let p2 = &polygon[j];

        // Check if point is on the edge
        if p1.x == p2.x {
            let edge_x = p1.x;
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            if point.x == edge_x && point.y >= min_y && point.y <= max_y {
                return true;
            }
        } else {
            let edge_y = p1.y;
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);

            if point.y == edge_y && point.x >= min_x && point.x <= max_x {
                return true;
            }
        }

        // Ray casting algorithm for point inside polygon
        if ((p1.y > point.y) != (p2.y > point.y)) &&
           (point.x < (p2.x - p1.x) * (point.y - p1.y) / (p2.y - p1.y) + p1.x) {
            inside = !inside;
        }
    }

    inside
}

fn rectangle_inside_polygon(tiles_area: &TilesArea, polygon: &Vec<Tile>) -> bool {
    let min_x = tiles_area.tile_a.x.min(tiles_area.tile_b.x);
    let max_x = tiles_area.tile_a.x.max(tiles_area.tile_b.x);
    let min_y = tiles_area.tile_a.y.min(tiles_area.tile_b.y);
    let max_y = tiles_area.tile_a.y.max(tiles_area.tile_b.y);

    let mut y = min_y;
    while y <= max_y {
        let mut x = min_x;
        while x <= max_x {
            let point = Tile { x, y };
            if !point_in_polygon(&point, polygon) {
                return false;
            }
            x += 1;
        }
        y += 1;
    }

    true
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

    let part1 = tiles_areas[0].area;

    let mut part2 = 0u64;
    for tiles_area in tiles_areas.iter() {
        if rectangle_inside_polygon(&tiles_area, &tiles) {
            part2 = tiles_area.area;
            break;
        }
    }

    (part1.to_string(), part2.to_string())
}

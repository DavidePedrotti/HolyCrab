use crate::app::MyApp;
use crate::grid::*;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile, TileType};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub(crate) fn match_color_to_type(tile_type: Option<TileType>) -> (u8, u8, u8, u8) {
    if let Some(tile_type) = tile_type {
        match tile_type {
            TileType::Grass => (124, 252, 0, 255),
            TileType::Street => (64, 64, 64, 255),
            TileType::ShallowWater => (127, 255, 212, 255),
            TileType::DeepWater => (0, 0, 128, 255),
            TileType::Sand => (242, 231, 109, 255),
            TileType::Hill => (105, 132, 55, 255),
            TileType::Mountain => (139, 69, 19, 255),
            TileType::Wall => (178, 34, 34, 255),
            TileType::Teleport(_) => (255, 0, 255, 255),
            TileType::Lava => (255, 69, 0, 255),
            TileType::Snow => (220, 220, 220, 255),
        }
    } else {
        (255, 255, 255, 255)
    }
}

pub(crate) fn match_color_to_content(content: &Content) -> (u8, u8, u8, u8) {
    match content {
        // match on all variants giving a rgba color for each
        Content::Rock(_) => (112, 128, 144, 255),
        Content::Tree(_) => (0, 100, 0, 255),
        Content::Garbage(_) => (0, 0, 0, 255),
        Content::Fire => (255, 0, 0, 255),
        Content::Coin(_) => (255, 215, 0, 255),
        Content::Bin(_) => (70, 130, 180, 255),
        Content::Crate(_) => (255, 128, 0, 255),
        Content::Bank(_) => (128, 128, 128, 255),
        Content::Water(_) => (173, 216, 230, 255),
        Content::Market(_) => (255, 0, 255, 255),
        Content::Fish(_) => (64, 224, 208, 255),
        Content::Building => (204, 85, 0, 255),
        Content::Bush(_) => (50, 205, 50, 255),
        Content::JollyBlock(_) => (255, 192, 203, 255),
        Content::Scarecrow => (160, 82, 45, 255),
        Content::None => (255, 255, 255, 255),
    }
}

pub(crate) fn save(grid: &Grid, path: PathBuf) {
    let serialized = bincode::serialize(grid).unwrap();
    //let path = std::path::Path::new("grid.json");
    let mut file = File::create(path).unwrap();
    file.write_all(&serialized).unwrap();
}

pub(crate) fn load_as_grid(grid: &mut Grid, path: PathBuf /*, generate_elevation: bool*/) {
    //let path = std::path::Path::new("grid.json");
    let deserialized: Grid = bincode::deserialize(&std::fs::read(path).unwrap()).unwrap();

    /*if generate_elevation {
        for cells in &mut grid.cells {
            for cell in cells {
                cell.elevation=0
            }
        }
    }*/

    *grid = deserialized;
}

pub(crate) fn load(
    path: PathBuf,
) -> (
    Vec<Vec<Tile>>,
    (usize, usize),
    EnvironmentalConditions,
    f32,
    Option<HashMap<Content, f32>>,
) {
    let mut grid = Grid::new(1);
    load_as_grid(&mut grid, path);
    let mut out = vec![
        vec![
            Tile {
                tile_type: TileType::Grass,
                elevation: 0,
                content: Content::None,
            };
            grid.size
        ];
        grid.size
    ];
    for i in 0..grid.size {
        for j in 0..grid.size {
            let tile = &grid.cells[i][j];
            out[i][j] = Tile {
                tile_type: tile.tile_type.unwrap(), // tile type is not empty, otherwise I cannot
                // call load
                elevation: tile.elevation,
                content: tile.content.clone(),
            };
        }
    }
    (
        out,
        grid.robot_pos,
        EnvironmentalConditions::new(&grid.weather, 15, 12).unwrap(), // weather forecast is never
        // empty
        grid.max_score,
        None,
    )
}

pub(crate) fn gui_start() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "WorldGen.unwrap().unwrap().unwrap()",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
        .unwrap();
}

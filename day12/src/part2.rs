use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, x: i32, y: i32) -> Option<char> {

        if x < 0 || y < 0 {
            return None;
        }

        Some(*self.rows.get(y as usize)?.get(x as usize)?)
    }
    pub fn get_with_pos(&self, pos: &Pos) -> Option<char> {
        self.get(pos.x as i32, pos.y as i32)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }

    pub fn new(input: &str) -> Self {
        let mut matrix = Matrix::default();

        for line in input.lines() {
            matrix.add_row(line.chars().collect());
        }
        matrix
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.rows[y][x] = c;
    }

    pub fn set_all_to(&mut self, input: char) {
        self.rows.iter_mut().for_each(|i| {
            i.iter_mut().for_each(|c| *c = input);
        });
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}

struct Plot {
    regions: Vec<Region>,
    raw_input: Matrix
}
impl Plot {
    pub fn print(&self, include_map: bool) {
        for region in self.regions.iter() {
            println!("Region: {}, perimeter: {}, area: {}", region.id, region.perimeter, region.positions.len());
            if include_map {
                region.print(self.raw_input.clone());
                println!();
                println!();
            }
        }
    }
}

enum Direction {
    Up,
    Right,
    Left,
    Down
}

struct Side {
    pos: Pos,
    dir: Direction,
}

struct Region {
    positions: Vec<Pos>,
    id: char,
    perimeter: u64,
    area: u64,
    vertical_sides: Vec<Side>,
    horizontal_sides: Vec<Side>,
}

impl Region {
    // print region, require a matrix input
    pub fn print(&self, mut map: Matrix) {

        // take matrix clone, empty it, and draw region positions
        map.set_all_to('.');

        for p in self.positions.iter() {
            map.set(p.x as usize, p.y as usize, self.id);
        }

        map.print();
    }
}

#[derive(Debug, Clone, Default, Copy)]
struct Pos {
    x: i32,
    y: i32,
    border: u64 // count of neighbours that is not identical
}
impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, border: 0 }
    }
}
impl PartialEq for Pos {
    fn eq(&self, other: &Pos) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn search(map: &Matrix, used: &mut Vec<Pos>, start: Pos, id: char) -> Vec<Pos> {
    let mut queue : VecDeque<Pos> = VecDeque::new();
    let mut region: Vec<Pos> = vec![];
    queue.push_back(start);

    // while queue has elements
    while let Some(mut pos) = queue.pop_front() {
        // if new element is different from current search value, or it has been visited before, we return
        if used.contains(&pos) {
            continue;
        }

        // if current position is == id
        if map.get_with_pos(&pos).unwrap() == id {

            // get the 4 neighbors, if some add to queue, if value is different increase border
            let search = vec![Pos::new(pos.x, pos.y - 1), Pos::new(pos.x + 1, pos.y), Pos::new(pos.x, pos.y + 1), Pos::new(pos.x - 1, pos.y)];

            // check neighbors
            for s in search.iter() {
                if let Some(p) = map.get_with_pos(s) {
                    // if id match, add to search
                    if p == id {
                        queue.push_back(*s);
                        continue;
                    } else {
                        // if neighbour, increase counter
                        pos.border += 1;
                        // add side
                        // todo
                        if s.x > pos.x {
                            todo!()
                        }
                    }
                } else {
                    // outside map, so an outside border
                    pos.border += 1;
                    // add side
                    // todo
                }
            }

            // part of this region
            region.push(pos);
            // has been used now
            used.push(pos);
        } else {
            // not part of this region, we continue
            continue;
        }
    }
    region
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // Change input into matrix
    let mut matrix = Matrix::new(input);
    let mut plot = Plot { regions: vec![], raw_input: matrix.clone() };

    // search containers
    let mut used: Vec<Pos> = vec![];

    // go through each position in matrix, check if it has been visited aka. is contained in Vec<Pos> used;
    //  if not visited, we start a new search from here and add to Vec<Pos> new_region and VecDeque<Pos> search
    // search goes in all 4 directions
    for (y, row) in matrix.rows.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let curr_pos = Pos {x: x as i32, y: y as i32, border: 0 };

            if used.contains(&curr_pos) {
                continue;
            }

            // if not a used position, we start a search to get this region
            let region_pos = search(&matrix, &mut used, curr_pos, *val);
            let region_border_sum = region_pos.iter().map(|p| p.border).sum::<u64>();
            let region_area = region_pos.len();
            let new_region = Region {id: *val, positions: region_pos, perimeter: region_border_sum, area: region_area as u64};

            plot.regions.push(new_region);
        }
    }

    // todo figure out how to get sides of a polygon based on points
    // when we add pos to a region, add sides too? Array for vertical and horizontal sides?
    // then we can sort horizontal sides by x then y
    // and vertical sides by y then x
    // vert_sides.sort_unstable_by_key(|side| (side.pos.y, side.pos.x));

    // that should gives us arrays sorted in a way that we can check if facing is different from last facing
    //  OR x != last_x OR y != last_y + 1

    // for vertical we do the same but swap x and y checks.









    // find total price by multiplying regions area and perimeter and summing them
    let price = plot.regions.iter().map(|p| p.perimeter * p.area).sum::<u64>();

    // print plots
    plot.print(false);


    Ok(price.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("436", process(input)?);
        Ok(())
    }
    #[test]
    fn test_process2() -> anyhow::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
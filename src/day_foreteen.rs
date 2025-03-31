#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}
impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_on_field(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub a: Vec2,
    pub b: Vec2,
}
impl Line {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    pub fn collides(&self, point: &Vec2) -> bool {
        if self.a.x == self.b.x {
            point.x == self.a.x
                && point.y + 1 >= self.a.y.min(self.b.y)
                && point.y + 1 <= self.a.y.max(self.b.y)
        } else {
            point.y == self.a.y
                && point.x >= self.a.x.min(self.b.x)
                && point.x <= self.a.x.max(self.b.x)
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Lines(Vec<Line>);

impl Lines {
    pub fn new(lines: Vec<Line>) -> Self {
        Self(lines)
    }

    pub fn collides(&self, point: &Vec2) -> bool {
        self.0.iter().any(|line| line.collides(point))
    }

    pub fn get_abyss(&self) -> usize {
        self.0.iter().max_by_key(|item| item.a.y).unwrap().a.y + 1
    }

    // pub fn drop(&self, point: &mut Vec2) {
    //     while !self.collides(point) && point.y < self.get_abyss() {
    //         point.move_on_field(point.x, point.y + 1);
    //     }
    //     // loop {
    //     //     point.move_on_field(point.x, point.y + 1);
    //     //     if self.collides(point) {
    //     //         point.move_on_field(point.x - 1, point.y + 1);
    //     //     }
    //     // }
    // }
}

pub fn drop_sand(lines: Lines, point: &mut Vec2) {
    // while !lines.collides(point) {
    while true {
        point.move_on_field(point.x, point.y + 1);
        if lines.collides(point) {
            // point.move_on_field(point.x - 1, point.y + 1);
            break;
        }
    }
    // loop {
    //     point.move_on_field(point.x, point.y + 1);
    //     if lines.collides(point) {
    //         point.move_on_field(point.x - 1, point.y + 1);
    //         // if lines.get_abyss() > point.y {
    //         //     break;
    //         // }
    //         break;
    //     } else {
    //         point.move_on_field(point.x, point.y + 1);
    //         // break;
    //     }
    // }
}

// fn visible_from_left(row: Vec<usize>, tree: usize) -> bool {}
mod test {
    use super::*;
    #[test]
    fn line_collides() {
        let l1 = Line::new(Vec2::new(1, 1), Vec2::new(1, 3));

        let dot1 = Vec2::new(1, 1);
        let dot2 = Vec2::new(1, 2);
        let dot3 = Vec2::new(1, 3);
        let dot_ok = Vec2::new(2, 3);

        assert!(l1.collides(&dot1), "dot1 not on line");
        assert!(l1.collides(&dot2), "dot2 not on line");
        assert!(l1.collides(&dot3), "dot3 not on line");
        assert!(!l1.collides(&dot_ok), "dot_ok on line");
    }
    #[test]
    fn lines_collides() {
        let lines = Lines::new(vec![
            Line::new(Vec2::new(1, 1), Vec2::new(1, 3)),
            Line::new(Vec2::new(1, 2), Vec2::new(5, 2)),
        ]);

        let dot1 = Vec2::new(1, 1);
        let dot2 = Vec2::new(2, 2);
        let dot_ok = Vec2::new(3, 1);

        assert!(lines.collides(&dot1y), "dot1 on line 1");
        assert!(lines.collides(&dot2), "dot2 on line 2");
        assert!(!lines.collides(&dot_ok), "dot_ok not on a line");
    }

    #[test]
    fn abyss() {
        let lines = Lines::new(vec![
            Line::new(Vec2::new(498, 4), Vec2::new(498, 6)),
            Line::new(Vec2::new(498, 6), Vec2::new(496, 6)),
            Line::new(Vec2::new(503, 4), Vec2::new(502, 4)),
            Line::new(Vec2::new(502, 4), Vec2::new(502, 9)),
            Line::new(Vec2::new(502, 9), Vec2::new(494, 9)),
        ]);

        assert_eq!(lines.get_abyss(), 10);
    }

    #[test]
    fn move_point() {
        let lines = Lines::new(vec![
            Line::new(Vec2::new(498, 4), Vec2::new(498, 6)),
            Line::new(Vec2::new(498, 6), Vec2::new(496, 6)),
            Line::new(Vec2::new(503, 4), Vec2::new(502, 4)),
            Line::new(Vec2::new(502, 4), Vec2::new(502, 9)),
            Line::new(Vec2::new(502, 9), Vec2::new(494, 9)),
        ]);
        let mut starting_dot = Vec2::new(500, 0);
        drop_sand(lines.clone(), &mut starting_dot);
        assert_eq!(starting_dot.y, 8);
        assert_eq!(starting_dot.x, 500);

        // let mut starting_dot_2 = Vec2::new(500, 0);
        // drop_sand(lines, &mut starting_dot_2);
        // assert_eq!(starting_dot_2.y, 8);
        // assert_eq!(starting_dot_2.x, 499);
    }
}

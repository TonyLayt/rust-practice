
use std::collections::HashSet;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub a: Point, // ліва верхня
    pub b: Point, // права нижня
}

pub fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_points = HashSet::new();

    for rect in xs {
        // знайдемо межі прямокутника
        let x_start = rect.a.x.min(rect.b.x);
        let x_end = rect.a.x.max(rect.b.x);
        let y_start = rect.a.y.min(rect.b.y);
        let y_end = rect.a.y.max(rect.b.y);

        // проходимо по всіх точках, які покриває прямокутник
        for x in x_start..x_end {
            for y in y_start..y_end {
                occupied_points.insert((x, y));
            }
        }
    }

    occupied_points.len() as i32

}

pub fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        }
    ]
}

#[test]
pub fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Test пройдено, зайнята площа: {}", occupied);
}
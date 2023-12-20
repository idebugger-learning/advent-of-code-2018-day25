use std::collections::BTreeSet;

type Point4 = (isize, isize, isize, isize);
type Constellation = BTreeSet<Point4>;

fn main() {
    let input = include_str!("../data/input.txt");

    let input = input
        .split('\n')
        .map(|row| row.trim().split(','))
        .map(|r| r.map(|num| num.parse::<isize>()))
        .map(|mut r| {
            (
                r.next().unwrap().unwrap(),
                r.next().unwrap().unwrap(),
                r.next().unwrap().unwrap(),
                r.next().unwrap().unwrap(),
            )
        })
        .collect::<Vec<(_, _, _, _)>>();

    println!("Input: {:?}", input);

    let mut constellations = BTreeSet::new();
    for point in input {
        add_point(&mut constellations, &point);
    }

    println!("Length: {}", constellations.len());
}

fn add_point(constellations: &mut BTreeSet<Constellation>, point: &Point4) {
    let suitable_constellations = constellations
        .clone()
        .into_iter()
        .filter(|constellation| is_close_enough(constellation, point))
        .collect::<Vec<_>>();
    let mut new_constellation = BTreeSet::new();

    for constellation in suitable_constellations {
        constellations.remove(&constellation);
        new_constellation.extend(constellation.clone());
    }
    new_constellation.insert(point.clone());
    constellations.insert(new_constellation);
}

fn is_close_enough(constellation: &Constellation, point: &Point4) -> bool {
    constellation
        .iter()
        .any(|const_point| distance(const_point, point) <= 3)
}

fn distance(a: &Point4, b: &Point4) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1) + a.2.abs_diff(b.2) + a.3.abs_diff(b.3)
}

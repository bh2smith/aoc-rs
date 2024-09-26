use itertools::Itertools;

#[derive(Debug)]
struct Ingredient {
    // how well it helps the cookie absorb milk
    capacity: i64,
    // how well it keeps the cookie intact when full of milk
    durability: i64,
    // how tasty it makes the cookie.
    flavor: i64,
    // how it improves the feel of the cookie
    texture: i64,
    // how many calories it adds to the cookie.
    calories: i64,
}

impl Ingredient {
    fn from(input: &str) -> Self {
        let input = input.replace(':', "");
        let input = input.replace(',', "");
        let items = input.split_whitespace().collect_vec();
        Self {
            capacity: items[2].parse().unwrap(),
            durability: items[4].parse().unwrap(),
            flavor: items[6].parse().unwrap(),
            texture: items[8].parse().unwrap(),
            calories: items[10].parse().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input.trim().split('\n').map(Ingredient::from).collect()
}

fn solve_4(ingredients: Vec<Ingredient>, count_cals: Option<i64>) -> i64 {
    assert_eq!(ingredients.len(), 4);
    let mut best = 0;
    for a in 0..100 + 1 {
        for b in 0..100 + 1 {
            for c in 0..100 + 1 {
                if a + b + c <= 100 {
                    let d = 100 - (a + b + c);
                    let x = &ingredients[0];
                    let y = &ingredients[1];
                    let z = &ingredients[2];
                    let w = &ingredients[3];
                    if let Some(cal_count) = count_cals {
                        if a * x.calories + b * y.calories + c * z.calories + d * w.calories
                            != cal_count
                        {
                            continue;
                        }
                    }

                    let m = [
                        a * x.capacity + b * y.capacity + c * z.capacity + d * w.capacity,
                        a * x.durability + b * y.durability + c * z.durability + d * w.durability,
                        a * x.flavor + b * y.flavor + c * z.flavor + d * w.flavor,
                        a * x.texture + b * y.texture + c * z.texture + d * w.texture,
                    ];

                    let value = m.iter().map(|&t| 0.max(t)).product();
                    if value > best {
                        best = value;
                    }
                }
            }
        }
    }
    best
}

fn solve_2(ingredients: Vec<Ingredient>, count_cals: Option<i64>) -> i64 {
    assert_eq!(ingredients.len(), 2);
    let mut best = 0;
    for a in 0..100 + 1 {
        let d = 100 - a;

        let x = &ingredients[0];
        let y = &ingredients[1];

        if let Some(cal_count) = count_cals {
            if a * x.calories + d * y.calories != cal_count {
                continue;
            }
        }
        let m = [
            a * x.capacity + d * y.capacity,
            a * x.durability + d * y.durability,
            a * x.flavor + d * y.flavor,
            a * x.texture + d * y.texture,
        ];

        let value = m.iter().map(|&t| 0.max(t)).product();
        println!("{a}-{d}, {m:?} => {value}");
        if value > best {
            best = value;
        }
    }
    best
}

pub fn puzzle1(input: &str) -> i64 {
    let ingredients = parse_input(input);
    match ingredients.len() {
        2 => solve_2(ingredients, None),
        4 => solve_4(ingredients, None),
        _ => unimplemented!(),
    }
}

pub fn puzzle2(input: &str) -> i64 {
    let ingredients = parse_input(input);
    match ingredients.len() {
        2 => solve_2(ingredients, Some(500)),
        4 => solve_4(ingredients, Some(500)),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 62842880);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}

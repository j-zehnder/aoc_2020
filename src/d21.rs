use std::collections::{HashSet, HashMap};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn new() -> Self {
        Self {
            ingredients: HashSet::new(),
            allergens: HashSet::new(),
        }
    }

    fn from_str(line: &str) -> Self {
        let mut s = line.split(" (contains ");
        let mut food = Food::new();

        let ingredients_str = s.next().unwrap();
        for i in ingredients_str.split(' ') {
            food.ingredients.insert(i.to_string());
        }

        let allergens_str = s.next().unwrap().replace(')', "");
        for allergen in allergens_str.split(", ") {
            food.allergens.insert(allergen.to_string());
        }

        food
    }
}
fn solve(foods: &[Food]) -> (usize, String) {
    let mut safe_ingredients: HashSet<String> = HashSet::new();
    let mut all_allergens: HashSet<String> = HashSet::new();

    for food in foods {
        for allergen in &food.allergens {
            all_allergens.insert(allergen.clone());
        }
        for ingredient in &food.ingredients {
            safe_ingredients.insert(ingredient.clone());
        }
    }

    let mut allergen_candidate_map: HashMap<String, HashSet<String>> = HashMap::new();

    for allergen in &all_allergens {
        allergen_candidate_map.insert(allergen.clone(), HashSet::new()); // prime empty

        let foods_containing_allergen = &foods
            .iter()
            .filter(|f| f.allergens.contains(allergen))
            .collect::<Vec<&Food>>();

        // list of ingredients of all foods containing the allergen
        let mut candidate_ingredients: HashSet<String> = HashSet::new();
        for food in foods_containing_allergen {
            for ingredient in &food.ingredients {
                candidate_ingredients.insert(ingredient.clone());
            }
        }

        for ingredient in candidate_ingredients {
            let mut in_all = true;
            for food in foods_containing_allergen {
                in_all = in_all && food.ingredients.contains(&ingredient);
            }

            // if an ingredient occurs in all foods with the same allergen,
            // it _could_ be the ingredient containing that allergen
            // this only works because any given ingredient can only be the source of 1 allergen
            // if the ingredient is not in all of the foods for a given allergen, it cannot be the source
            if in_all {
                safe_ingredients.remove(&ingredient);
                allergen_candidate_map
                    .get_mut(allergen)
                    .unwrap()
                    .insert(ingredient.clone());
            }
        }
    }

    // at this point,
    // - safe_ingredients is filtered to the ingredients that cannot be a source of allergen
    // - allergen_candidate_map is a mapping of allergen -> list of possible sources of that allergen

    //====================================================
    // part 1: count all occurrences of safe ingredients across all food's ingredients
    let p1 = foods
        .iter()
        .cartesian_product(&safe_ingredients)
        .filter(|f| f.0.ingredients.contains(f.1))
        .count();

    //====================================================
    // part2: figure out which ingredient is the source of each allergen
    // sort ingredient list _by allergen name_ and string them together
    let mut solved: HashMap<String, String> = HashMap::new();
    while solved.len() < all_allergens.len() {
        // while we have not solved all allergens
        let mut found = false;
        let mut found_allergen: String = String::new();
        for (allergen, candidates) in &allergen_candidate_map {
            if candidates.len() == 1 {
                found = true;
                found_allergen = allergen.clone();
                solved.insert(allergen.clone(), candidates.iter().next().unwrap().clone());
            }
        }

        // have to do this outside of the previous iteration,
        // you cannot modify the thing you are iterating over during iteration
        if found {
            allergen_candidate_map.remove(&found_allergen);
            for candidates in allergen_candidate_map.values_mut() {
                candidates.remove(solved.get(&found_allergen).unwrap());
            }
        } else {
            panic!("no allergens can be solved, avoiding an infinite loop")
        }
    }

    let mut sorted_keys = solved.keys().into_iter().cloned().collect::<Vec<String>>();
    sorted_keys.sort();

    let mut canonical = String::new();
    for k in sorted_keys {
        canonical = format!("{},{}", canonical, solved.get(&k).unwrap())
    }

    (p1, canonical[1..].to_string())
}

#[aoc_generator(day21)]
pub fn parse_input(input: &str) -> Vec<Food> {
    input.lines().map(Food::from_str).collect()
}

#[aoc(day21, part1)]
pub fn part1(foods: &[Food]) -> usize {
    solve(foods).0
}

#[aoc(day21, part2)]
pub fn part2(foods: &[Food]) -> String {
    solve(foods).1
}


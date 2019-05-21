// -- SymSpell --
// Explanation at
// https://medium.com/@wolfgarbe/1000x-faster-spelling-correction-algorithm-2012-8701fcd87a5f

// TL;DR, HashTable keys are generated
// from all words + all possible
// permutations of those words with up
// to two deletes, and the data held
// in each key is the correctly spelled
// word (or possible words) with their
// count included to determine which
// of the possible words is more likely.

use std::collections::HashMap;

// word == word
// score == word priority // higher number == higher priority
#[derive(Debug, Clone)]
struct Word {
    word: String,
    score: u64
}

// word_map must be populated after
// Dictionary struct is created.
// word_map is the reference dictionary.
// All entries here are considered correct
// error_map is the compiled list
// of acceptable permutations.
// word_map is searched first for inputs.
// if none are found, then
// error_map is then searched for possible matches
#[derive(Debug, Clone)]
struct Dictionary {
    word_map: HashMap<String, Word>,
    error_map: HashMap<String, Vec<String>>,
    error_distance: u8
}
// UNIMPLEMENTED YET
// only counts in word_map are measured to
// determined probable match
// only counts in word_map are incremented
// and check when inserting a new word.
// counts in error_map are ignored.

// only necessary for word_map,
// only word_map requires knowing
// the word score.
// error_map can be Hash<String, Vec<String>>
impl Word {
    fn new(word: &str, count: u64) -> Word {
        Word {
            word: word.to_string(),
            score: count
        }
    }
}


impl Dictionary {
    fn new() -> Dictionary {
        Dictionary {
            word_map: HashMap::new(),
            error_map: HashMap::new(),
            error_distance: 2
        }
    }

    fn insert(&mut self, word: &str) {
        if let Some(x) = self.word_map.get_mut(word) {
            x.score += 1;
        } else {
            self.word_map
            .insert(
                word.to_string(),
                Word::new(word, 1)
            );
        }
    }

    fn insert_with_count(&mut self, word: &str, count: u64) {
        self.insert(word);

        self.word_map
        .get_mut(word)
        .unwrap()
        .score = count;
    }

    // Permutations
    // inserted don't replace the
    // existing permutations,
    // they are only are
    // appended to the existing
    // values.
    fn insert_with_permutations(&mut self, word: &str) {
        if let Some(_x) = self.word_map.get_mut(word) {
            self.add_permutations(word);
        } else {
            self.insert(word);// insert new word.
            self.add_permutations(word);
        }

    }

    fn insert_with_permutations_and_count(&mut self, word: &str, count: u64) {
        if let Some(x) = self.word_map.get_mut(word) {
            x.score = count;
            self.add_permutations(word);
        } else {
            self.insert_with_count(word, count);// insert new word.
            self.add_permutations(word);
        }

    }

    fn add_permutations(&mut self, word: &str) {
        // Vec<String>
        // Must only contain inserts of
        // correct words
        let permuted_keys = self.permutations_of(word);
        for i in permuted_keys {
            // if error key exists
            if let Some(x) = self.error_map.get_mut(&i) {
                let mut new_set: HashMap<String, ()> = HashMap::new();
                // collect vector of existing
                // correct possibilities
                // into hashmap to prevent
                // duplicate entries
                for y in x.clone() {
                    new_set.insert(y, ());
                }
                // Add the new word to
                // list of correct
                // possibilities
                // at this key
                new_set.insert(word.to_string(), ());
                x.clear();
                for j in new_set.keys() {
                    x.push(j.to_string());
                }
            } else {
                self.error_map
                .insert(
                    i.clone(),
                    vec![word.to_string()]
                );
            }
        }
    }

    fn generate_permutations(&self, word: &str) -> Vec<String> {
        let mut permutations: Vec<String> = Vec::new();
        // Generate permutations of this word
        for i in 0..word.len() {
            let mut permuted: Vec<char> = word.chars().collect();
            permuted.remove(i);
            permutations.push(permuted.into_iter().collect::<String>());
        }
        permutations
    }

    fn permutations_of(&self, word: &str) -> Vec<String> {
        let mut permutation_list: HashMap<String, ()> = HashMap::new();
        permutation_list.insert(word.to_string(), ());
        for _i in 0..self.error_distance {
            for u in permutation_list.clone().keys() {
                for o in self.generate_permutations(u) {
                    permutation_list.insert(o, ());
                }
            }
        }
        let mut result: Vec<String> = Vec::new();
        for i in permutation_list.keys() {
            result.push(i.to_string());
        }
        result
    }


    fn find_best_match(&self, possibilities: Vec<String>) -> String {
        let mut max = 0;
        let mut best_match: Option<String> = None;
        for i in possibilities.clone() {
            let score = self.word_map[&i].score;
            if score > max {
                max = score;
                best_match = Some(i.clone());
            }
        }
        best_match.expect("Nothing matched in iterator... somehow...")
    }


    fn generate_errors(&mut self) {
        let mut result = self.error_map.clone();

        // word_map: HashMap<String, Word>
        // error_map: HashMap<String, Vec<String>>
        let error_map = if self.error_map.is_empty() {
            // Word -> Vec<String>
            // Word == .word : String
            // but Word is behind a HashMap key...
            // So we iterate and convert it
            // to Vec<String>
            let mut words: HashMap<String, Vec<String>> = HashMap::new();
            for s in self.word_map.clone().keys() {
                words.insert(s.to_string(), vec![s.to_string()]);
            }
            words // Vec<String>
        } else {
            self.error_map.clone() // Vec<String>
        };

        for i in error_map.keys() {
            if i.len() > 2 {
                for u in 0..i.len() {
                    let mut permuted: Vec<char> = i.chars().collect();
                    permuted.remove(u);
                    let permutation = permuted.into_iter().collect::<String>();

                    if let Some(x) = result.get_mut(&permutation) {
                        let mut set: HashMap<String, ()> = HashMap::new();
                        for w in x.clone() {
                            set.entry(w.clone()).or_insert(());
                        }
                        // for w in error_map.get(i).unwrap().clone() {
                        //     set.entry(w.word).or_insert(());
                        // }
                        let mut y: Vec<String> = Vec::new();
                        for k in set.keys() {
                            y.push(k.to_string());
                        }
                        x.clear();
                        for v in y {
                            x.push(v);
                        }
                    } else {
                        result
                        .entry(permutation)
                        .or_insert(error_map.get(i).unwrap().clone());
                    }
                }
            }
        }
        self.error_map = result;
    }

    fn check(&self, word: &str) -> Option<String>{
        // regular functions don't capture parent scope.
        // closures do catch parent scope
        let find = |word: &str| -> Option<String> {
            if let Some(x) = self.word_map.get(word) {
                Some(x.word.clone())
            } else if let Some(x) = self.error_map.get(word) {
                if x.len() > 1 {
                    Some(self.find_best_match(x.to_vec()))
                } else {
                    Some(x[0].clone())
                }
            } else {
                None
            }
        };

        if let Some(x) = find(word) {
            return Some(x);
        }

        let mut permutations = vec![word.to_string()];
        permutations.extend(self.permutations_of(word));
        for v in permutations.clone() {
            permutations.extend(self.permutations_of(&v));
        }

        for i in permutations {
            if let Some(x) = find(&i) {
                return Some(x);
            }
        }
        return None;
    }
}

fn main() {
    let mut d = Dictionary::new();
    // d.insert_with_permutations("Fork");
    // d.insert_with_permutations("Doofus");
    // d.insert_with_permutations_and_count("Bell", 32);
    // d.insert_with_permutations_and_count("Belly", 29);
    // d.insert_with_permutations_and_count("Bellow", 19);
    // println!("{:?}", d.generate_permutations("Bell"));
    // println!("{:?}", "===");
    // println!("{:?}", d.generate_permutations("Belly"));
    // println!("{:?}", "===");
    // for i in d.word_map.clone() {
    //     println!("{:?}", i);
    // }
    // println!("");
    // for i in d.error_map.clone() {
    //     println!("{:?}", i);
    // }
    // println!("");
    // println!("{:?}", d.check("Dofus"));
    // println!("{:?}", d.check("Dfus"));
    // println!("{:?}", d.check("Doooofus"));
    // println!("{:?}", d.check("Dooofus"));
    // println!("{:?}", d.check("Forky"));
    // println!("{:?}", d.check("Forkyy"));
    // println!("{:?}", d.check("Fo"));
    // println!("Hello, world!");

    // Testing setup
    use std::io;
    loop {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("no work... >:(");
        if cmd.trim() == "add" {
            let mut word = String::new();
            let mut value = String::new();
            println!("word? ");
            io::stdin().read_line(&mut word).expect("no work... >:(");
            println!("value? ");
            io::stdin().read_line(&mut value).expect("no work... >:(");
            d.insert_with_permutations_and_count(word.trim().as_ref(), value.trim().parse().expect("not a number"));
        } else {
            match d.check(&cmd.trim()) {
                Some(x) => println!("Did you mean {}?", x),
                _ => println!("Not found :(")
            };
        }
    }
}

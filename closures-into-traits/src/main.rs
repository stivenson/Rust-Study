use std::{fmt, fs};

#[derive(Debug)]
struct TupleResultSearch {
    amount: u64,
    word: String,
}

impl fmt::Display for TupleResultSearch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "word: {:?}, amount: {:?}", self.word, self.amount)
    }
}

struct CacherFindText<T> // "Struct" with generics to save data of process
    where T: Fn(&str) -> u64 // add structure to generic: "T". This structure is the "closure"
{
    heavy_process: T, // To save the "closure" with heavy logic
    last_result: Option<TupleResultSearch>, // Result of last search
    preview_results: Vec<Option::<TupleResultSearch>>, // To save search results and be able to reuse them
}

impl<T> CacherFindText<T> // Implementation of process to search words and data caching
    where T: Fn(&str) -> u64
{
    fn new(heavy_process: T) -> CacherFindText<T> { // Init data
        CacherFindText {
            heavy_process,
            last_result: None,
            preview_results: Vec::new()
        }
    }

    fn update_last_result(&mut self, arg: &str, v: &u64) {
        self.last_result = Some(
            TupleResultSearch {
                amount: *v,
                word: arg.to_string()
            }
        );
    }

    fn run_heavy_process_and_get_result(&mut self, arg: &str) -> u64 {
        let v = (&self.heavy_process)(arg);
        &self.update_last_result(arg, &v);
        &self.preview_results.push(Some(
                TupleResultSearch {
                    amount: v,
                    word: arg.to_string()
                }
            ));
        v
    }

    fn get_cache_result(&mut self, index: usize, arg: &str) -> u64 {
        if let Some(op) = &self.preview_results[index] {
            self.last_result = Some(
                TupleResultSearch {
                    amount: op.amount,
                    word: arg.to_string()
                }
            );
            op.amount
        } else {
            self.run_heavy_process_and_get_result(arg)
        }
    }

    fn search(&mut self, arg: &str) -> u64 {
        match &self.last_result {
            Some(v) => {
                if v.word == arg {
                    v.amount
                } else {
                    let index_cache = &self.preview_results.iter().position(|r| {
                        match r {
                            Some(xt) => &xt.word == arg,
                            None => false
                        }
                    });

                    match index_cache {
                        Some(index) => self.get_cache_result(*index, arg),
                        None => self.run_heavy_process_and_get_result(arg)
                    }
                }
            },
            None => self.run_heavy_process_and_get_result(arg)
        }
    }
}



fn find_words(words_to_find: [&str; 20]) { // Use of implementation to search words and data caching

    // Use of "Struct" with generics to run "closure with heavy logic" and save results in cache(an array of results)
    let mut expensive_result = CacherFindText::new(|word| {
        
        // Heavy logic //

        let fname = "text_to_find.txt";
        // Contents of file
        let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");
        // Search a word into file's content
        match contents.find(word) {
            Some(v) => v as u64, 
            None => 0
        }

    });

    // Apply search to each word
    for word in &words_to_find {
        // expensive_result.search will search word in file or get result from cache if the search for the word has already been made.
        println!("Word {:?} found in position: {:?} into file.", word, &expensive_result.search(word)); 
    }
}

fn main() {
    // array of words to search into file.
    let words_to_find: [&str; 20] = ["ddddd", "oportere", "oportere", "oportere", "desconocida" ,"sit", "meiuuuuu", "meiuuuuu", "meiuuuuu", "meiuuuuu", "meiuuuuu", "desconocida", "desconocida", "dddddaaaa", "instructiorrrrr", "instructiorrrrr", "instructiorrrrr", "oportere", "desconocida", "dddddaaaa"];
    // Run logic
    find_words(words_to_find);
}

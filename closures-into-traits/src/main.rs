use std::{fmt, fs};

#[derive(Debug, Clone)]
struct SearchResult {
    position: u64,
    word: String,
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "word: '{}', position: {}", self.word, self.position)
    }
}

/// Generic cacher that stores results of expensive operations
struct CacherFindText<T>
where
    T: Fn(&str) -> u64,
{
    heavy_process: T,
    last_result: Option<SearchResult>,
    cached_results: Vec<SearchResult>,
}

impl<T> CacherFindText<T>
where
    T: Fn(&str) -> u64,
{
    fn new(heavy_process: T) -> Self {
        CacherFindText {
            heavy_process,
            last_result: None,
            cached_results: Vec::new(),
        }
    }

    fn update_last_result(&mut self, word: &str, position: u64) {
        self.last_result = Some(SearchResult {
            position,
            word: word.to_string(),
        });
    }

    fn run_process_and_cache(&mut self, word: &str) -> u64 {
        let position = (self.heavy_process)(word);
        
        let result = SearchResult {
            position,
            word: word.to_string(),
        };
        
        self.last_result = Some(result.clone());
        self.cached_results.push(result);
        
        position
    }

    fn check_cache(&mut self, word: &str) -> Option<u64> {
        self.cached_results
            .iter()
            .find(|r| r.word == word)
            .map(|r| {
                self.last_result = Some(r.clone());
                r.position
            })
    }

    fn search(&mut self, word: &str) -> u64 {
        // Check last result first
        if let Some(ref last) = self.last_result {
            if last.word == word {
                return last.position;
            }
        }

        // Check cache
        if let Some(position) = self.check_cache(word) {
            return position;
        }

        // Run expensive operation
        self.run_process_and_cache(word)
    }
}



fn find_words(words_to_find: &[&str]) {
    println!("=== Closures and Caching Example ===\n");

    // Create a cacher with a closure that performs expensive file I/O
    let mut cacher = CacherFindText::new(|word| {
        // Expensive operation: read file and search for word
        let filename = "text_to_find.txt";
        let contents = fs::read_to_string(filename)
            .expect("Failed to read file");
        
        contents.find(word).map(|v| v as u64).unwrap_or(0)
    });

    // Search for each word (results are cached automatically)
    for word in words_to_find {
        let position = cacher.search(word);
        if position > 0 {
            println!("Word '{}' found at position: {}", word, position);
        } else {
            println!("Word '{}' not found", word);
        }
    }

    println!("\nCache statistics:");
    println!("Total cached results: {}", cacher.cached_results.len());
}

fn main() {
    // Array of words to search (includes duplicates to demonstrate caching)
    let words_to_find = [
        "oportere",
        "oportere", // duplicate - will use cache
        "oportere", // duplicate - will use cache
        "sit",
        "desconocida",
        "desconocida", // duplicate - will use cache
        "unknown_word",
        "oportere", // duplicate - will use cache
    ];

    find_words(&words_to_find);
}

use std::cmp::Reverse;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("{:?}", find_synonyms("mesa"));
}

fn find_synonyms(term: &str) -> Vec<String> {
    let mut file = File::open("dict/th_es_ANY_v2.dat").expect("Failed opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let synonyms = match contents.find(format!("\n{}", term).as_str()) {
        Some(pos) => read_synonyms(&contents, pos),
        None => vec![],
    };
    synonyms
}

fn read_synonyms(contents: &str, position: usize) -> Vec<String> {
    let mut synonyms: Vec<_> = contents[position + 1 ..].lines().skip(1)
        .take_while(|line| &line[..1] == "-" || &line[..1] == "(")
        .flat_map(|line| line.split("|").skip(1))
        .map(String::from).collect();
    synonyms.sort_by_key(|word| Reverse(word.len()));
    synonyms.dedup();
    synonyms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_synonyms_for_a_known_term() {
        assert_eq!(vec!["izquierdo", "siniestro"], find_synonyms("zurdo"));
        assert_eq!(vec!["retrato", "foto"], find_synonyms("fotografía"));
        assert_eq!(vec!["zarzamora"], find_synonyms("zarza"));
    }

    #[test]
    fn it_reads_synonyms_in_multiple_lines() {
        assert_eq!(vec!["abundante", "copioso", "suelto"], find_synonyms("granel"));
    }

    #[test]
    fn it_sorts_by_longest_first_and_removes_duplicates() {
        assert_eq!(vec!["arcilla", "marga", "gres"], find_synonyms("greda"));
    }
}

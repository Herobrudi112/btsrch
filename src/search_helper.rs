use std::time::Instant;

pub fn search<T>(query: &String, list: Vec<(String, &T)>) -> Vec<(usize, Vec<usize>)> {
    if query.len()==0{
        return (0..list.len()).map(|x| (x, Vec::new())).collect::<Vec<_>>();
    }
    let start=Instant::now();
    let lower_case = query.to_lowercase();
    let non_found_ids = (0..list.len()).collect::<Vec<usize>>();
    let mut s_anywhere_whole = non_found_ids
        .clone()
        .into_iter()
        .filter_map(|i| {
            list[i].0.to_lowercase().find(&lower_case).map(|h| {
                let (_, lookup) = to_lowercase_lookup(&list[i].0);
                let h2 = lookup[h];
                let h3 = lookup[h + lower_case.len()];
                (i, vec![h2, h3])
            })
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    let mut non_found_ids=s_anywhere_whole.iter().map(|(i, _)| *i).collect::<Vec<_>>();
    let seperator_chars = vec![' ', '.', '-', '_', ';', ',']
        .drain(..)
        .filter(|c| !lower_case.contains(&c.to_lowercase().to_string()))
        .collect::<Vec<char>>();
    let mut s_between_spaces = non_found_ids
        .clone()
        .into_iter()
        .filter_map(|i| {
            split_multiple(&seperator_chars, list[i].0.clone())
                .iter()
                .find_map(|s| {
                    (s.1.to_lowercase() == *lower_case).then(|| (i, vec![s.0, s.0 + s.1.len()]))
                })
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    remove_found(&mut non_found_ids, &mut s_between_spaces);
    s_between_spaces.sort_by_key(|(_, h)| h[0]);
    let mut s_after_space = non_found_ids
        .clone()
        .into_iter()
        .filter_map(|i| {
            split_multiple(&seperator_chars, list[i].0.clone())
                .iter()
                .find_map(|s| {
                    (s.1.to_lowercase().starts_with(&lower_case)).then(|| {
                        let (_, lookup) = to_lowercase_lookup(&s.1);
                        let h3 = lookup[lower_case.len()];
                        (i, vec![s.0, s.0 + h3])
                    })
                })
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    remove_found(&mut non_found_ids, &mut s_after_space);
    s_after_space.sort_by_key(|(_, h)| h[0]);
    s_anywhere_whole.sort_by_key(|(_, h)| h[0]);
    println!("{:?}", start.elapsed());
    let mut temp=s_between_spaces
        .drain(..)
        .chain(s_after_space.drain(..)).collect::<Vec<_>>();
    let mut s_anywhere_whole=s_anywhere_whole.drain(..).filter(|x| temp.iter().find(|y| y.0==x.0).is_none()).collect::<Vec<_>>();
        temp.drain(..).chain(s_anywhere_whole.drain(..))
        .collect()
}
fn remove_found(non_found_ids: &mut Vec<usize>, s_at_start: &mut Vec<(usize, Vec<usize>)>) {
    *non_found_ids = non_found_ids
        .drain(..)
        .filter(|i| s_at_start.binary_search_by_key(i, |(a, _)| *a).is_err())
        .collect::<Vec<usize>>();
}
pub fn split_multiple(seperator_chars: &Vec<char>, mut string: String) -> Vec<(usize, String)> {
    let binding = string.clone();
    let mut t = binding
        .match_indices(|c| seperator_chars.contains(&c))
        .map(|(i, _)| i)
        .rev()
        .map(|i| {
            let t = string.split_off(i + 1);
            string.truncate(string.len() - 1);
            (string.len() + 1, t)
        })
        .collect::<Vec<(usize, String)>>();
    t.push((0, string));
    t.reverse();
    t
}
fn to_lowercase_lookup(s: &String) -> (String, Vec<usize>) {
    let mut string = String::new();
    let mut indices = vec![0];
    for c in s.chars() {
        let prev = *indices.last().unwrap();
        for _ in 0..(c.len_utf8() - 1) {
            indices.push(usize::MAX);
        }
        let lowercase = c.to_lowercase();
        indices.push(prev + lowercase.to_string().len());
        string.push(c);
    }
    (string, indices)
}
/*
todo:
for search term cinnamonn:
cinnamon-wayland
org.cinnamon.screensaver
cinnamon2d

for search term cinnamon screensaver:
cinnamon-screensaver



search for exact string at start, after+bevore/after whitespace/punctuation/...
recursive call for whitespace-split string? How do I sort that?
*/

pub fn search<T>(query: String, list: Vec<(String, &T)>) -> Vec<(usize, Vec<usize>)> {
    let mut s_at_start = (0..list.len())
        .into_iter()
        .filter_map(|i| {
            list[i]
                .0
                .starts_with(&query)
                .then(|| (i, vec![0, query.len()]))
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    s_at_start.sort_by_key(|(a, _)| &list[*a].0);
    let seperator_chars = vec![' ', '.', '-', '_']
        .drain(..)
        .filter(|c| !query.contains(*c))
        .collect::<Vec<char>>();
    let mut s_between_spaces = (0..list.len())
        .into_iter()
        .filter_map(|i| {
            split_multiple(&seperator_chars, list[i].0.clone())
                .iter()
                .find_map(|s| (s.1 == query).then(|| s.0))
                .map(|h| (i, vec![h, h + query.len()]))
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    s_between_spaces.sort_by_key(|(_, h)| h[0]);
    let mut s_after_space = (0..list.len())
        .into_iter()
        .filter_map(|i| {
            split_multiple(&seperator_chars, list[i].0.clone())
                .iter()
                .find_map(|s| (s.1.starts_with(&query)).then(|| s.0))
                .map(|h| (i, vec![h, h + query.len()]))
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    s_after_space.sort_by_key(|(_, h)| h[0]);
    let mut s_anywhere_whole = (0..list.len())
        .into_iter()
        .filter_map(|i| {
            list[i]
                .0
                .find(&query)
                .map(|h| (i, vec![h, h + query.len()]))
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    s_anywhere_whole.sort_by_key(|(_, h)| h[0]);
    s_at_start
        .drain(..)
        .chain(s_after_space.drain(..))
        .chain(s_anywhere_whole.drain(..))
        .collect()
}
pub fn split_multiple(seperator_chars: &Vec<char>, mut string: String) -> Vec<(usize, String)> {
    let binding = string.clone();
    let mut t=binding
        .match_indices(|c| seperator_chars.contains(&c))
        .map(|(i, _)| i)
        .rev()
        .map(|i| {
            let t = string.split_off(i + 1);
            string.truncate(string.len() - 1);
            (string.len()-1, t)
        }).collect::<Vec<(usize, String)>>();
        t.push((0, string));
        t.reverse();
        t
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

pub fn search<T>(query:String,list:Vec<(String, &T)>)->Vec<(usize, Vec<usize>)>{
    let mut s_at_start=(0..list.len()).into_iter().filter_map(|i| list[i].0.starts_with(&query).then(|| (i,vec![0, query.len()]))).collect::<Vec<(usize,Vec<usize>)>>();
    s_at_start.sort_by_key(|(a,_)| &list[*a].0);
    let mut s_after_space=(0..list.len()).into_iter().filter_map(|i| list[i].0.find(&format!(" {query}")).map(|h| (i, vec![h, h+query.len()]))).collect::<Vec<(usize,Vec<usize>)>>();
    s_after_space.sort_by_key(|(_, h)| h[0]);
    let mut s_anywhere_whole=(0..list.len()).into_iter().filter_map(|i| list[i].0.find(&query).map(|h| (i, vec![h, h+query.len()]))).collect::<Vec<(usize,Vec<usize>)>>();
    s_anywhere_whole.sort_by_key(|(_, h)| h[0]);
    s_at_start.drain(..).chain(s_after_space.drain(..)).chain(s_anywhere_whole.drain(..)).collect()
}
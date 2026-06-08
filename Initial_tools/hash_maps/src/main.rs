use std::collections::HashMap;

fn main() {

    let mut map: HashMap<String, Vec<i32>> = HashMap::new();

    map.insert("A".to_string(), vec![1,2,3]);
    map.insert("B".to_string(), vec![11,12,13]);

    for i in 0..21
    {
        if i <= 10
        {
            if !map["A"].contains(&i)
            {
                map.get_mut("A").unwrap().push(i);
            }
        }
        else
        {
            if !map["B"].contains(&i)
            {
                map.get_mut("B").unwrap().push(i);
            }
        }
    }
    for (key, value) in &map
    {
        println!("{}: {:?}", key, value);
    }
}

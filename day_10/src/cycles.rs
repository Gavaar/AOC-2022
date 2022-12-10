pub fn calc(input: &'static str) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::from([0, 1]);

    for line in input.lines() {
        let mut single_input = line.split(" ");
        let command = single_input.next().unwrap();
        let amount = match single_input.next(){
            Some(v) => v.parse::<i32>().unwrap(),
            None => 0,
        };

        let last_value = match values.last() {
            Some(v) => *v,
            None => 1,
        };

        if command == "noop" {
            values.push(last_value);
            continue;
        }
        
        let new_value = last_value + amount;
        values.push(last_value);
        values.push(new_value);
    }

    return values;
}

pub fn into_signal_strength(input: &Vec<i32>) -> Vec<i32> {
    input.iter().enumerate().map(|(index, value)| index as i32 * value).collect()
}
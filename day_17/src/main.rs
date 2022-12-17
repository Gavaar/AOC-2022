mod files;
mod rocks;
mod cave;
mod commands;

fn part_one(test: bool) {
    let (rocks_parsed, file) = files::get(test);
    let mut commands = commands::create(file);
    let mut cave = cave::create();
    let mut rock_thrower = rocks::get_new_rock_thrower(rocks_parsed);

    loop {
        if cave.falling_rock.is_none() {
            let rock = rock_thrower.next();
            cave.insert_rock(&rock);
        }

        if rock_thrower.called() == 2023 {
            println!("{}", rock_thrower.called());
            break;
        }
        
        let command = commands.next();
        // println!("calling {}", command);
        cave.act_on(command);
    }
    
    // println!("{:#?}", cave);
    println!("");
    // println!("I lost height {}", 3068 - cave.height());
    // println!("I did {} commands", commands.curr());
    println!("The height is: {}", cave.height() + 1);
}

fn main() {
    part_one(true);
    part_one(false);
}

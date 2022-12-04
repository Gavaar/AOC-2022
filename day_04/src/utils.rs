pub fn contained_by_partner(section_vec: &Vec<Vec<&'static str>>, overlaps: bool) -> Vec<bool> {
    let mut contained_by_partner: Vec<bool> = Vec::new();

    for section in section_vec {
        let mut first_elf = section[0].splitn(2, '-');
        let mut second_elf = section[1].splitn(2, '-');

        let first_start = first_elf.next().unwrap().parse::<i32>().unwrap();
        let first_end = first_elf.next().unwrap().parse::<i32>().unwrap();
        let second_start = second_elf.next().unwrap().parse::<i32>().unwrap();
        let second_end = second_elf.next().unwrap().parse::<i32>().unwrap();

        let contained = (first_end >= second_end && first_start <= second_start)
            || (second_end >= first_end && second_start <= first_start);
        let overlaps_value = !((first_start > second_start
            && first_start > second_end
            && first_end > second_start
            && first_end > second_end)
            || (first_start < second_start
                && first_start < second_end
                && first_end < second_start
                && first_end < second_end));
        if !overlaps_value { println!("[{}-{}] [{}-{}]", first_start, first_end, second_start, second_end) }
        contained_by_partner.push(if overlaps { overlaps_value } else { contained });
    }

    return contained_by_partner;
}

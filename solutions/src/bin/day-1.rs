use solutions::input::file_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let str_vec = file_input("./day1.txt");
    let mut res = 0;
    for str1 in str_vec {
        let mut res_str = vec!();
        for i in str1.chars() {
            if i.is_digit(10) {
                res_str.push(i);
                break;
            }
        }
        for i in str1.chars().rev() {
            if i.is_digit(10) {
                res_str.push(i);
                break;
            }
        }
        let str1: String = res_str.into_iter().collect();
        res += str1.parse::<i32>().unwrap();
    }
    println!("{:?}", res);
}

fn part2() {
    let str_vec = file_input("./day1.txt");
    let mut res = 0;
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for str1 in str_vec {
        let mut line = str1;
        let mut chr_vec = vec!();
        while !nums.iter().any(|x| line.starts_with(x)) && !line.chars().next().unwrap().is_numeric() {
            line = line[1..].to_string();
        }
        while !nums.iter().any(|x| line.ends_with(x)) && !line.chars().last().unwrap().is_numeric(){
            line = line[..line.len() - 1].to_string();
        }

        if line.chars().next().unwrap().is_numeric() {
            chr_vec.push(line.chars().next().unwrap());
        } else {
            for i in 0..nums.len() {
                if line.starts_with(nums[i]) {
                    chr_vec.push((i+1).to_string().chars().next().unwrap());
                }
            }
        }

        if line.chars().last().unwrap().is_numeric() {
            chr_vec.push(line.chars().last().unwrap());
        } else {
            for i in 0..nums.len() {
                if line.ends_with(nums[i]) {
                    chr_vec.push((i+1).to_string().chars().next().unwrap());
                }
            }
        }
        let str_res:String = chr_vec.into_iter().collect();
        res += str_res.parse::<i32>().unwrap();
    }
    println!("{:?}", res);
}
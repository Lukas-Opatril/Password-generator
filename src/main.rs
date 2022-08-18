use std::env;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::str;
fn main() {
    println!("Password text file basic generator");
    println!("To abort program, press Ctrl + C in Your console.");

    static mut BOOL_EXIT: bool = false;

    let mut argument = String::new();
    let mut range_1: i8 = 0;
    let mut range_2: i8 = 0;
    let mut only_range: i8 = 0;
    let file = File::create("list.txt").expect("Failed to create a text file!");
    let character_array = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2',
        '3', '4', '5', '6', '7', '8', '9',
    ];
    let mut length: u64 = 0;
    let args: Vec<String> = env::args().collect();

    ctrlc::set_handler(|| unsafe {
        BOOL_EXIT = true;
    })
    .expect("Error setting handler");

    if args.len() > 1 {
        argument = args[1].clone();

        if argument.contains("-range") {
            if args.len() < 4 {
                println!("You didn't specify the range!");
                exit(0);
            } else {
                range_1 = args[2].parse().expect("Failed to parse range arguments");
                range_2 = args[3].parse().expect("Failed to parse range arguments");

                if range_1 > range_2 {
                    println!("Beginning range can't be bigger then ending range");
                    exit(0);
                }
                if range_1 < 1 || range_2 < 1 {
                    println!("Your paswords can't be 0 length");
                    exit(0);
                }
                if range_1 > 6 || range_2 > 6 {
                    println!("Maximum range is 6!");
                    exit(0);
                }
            }
        } else if argument.contains("-only") {
            if args.len() < 3 {
                println!("You didn't specify the password length!");
                exit(0);
            } else {
                only_range = args[2].parse().expect("Failed to parse");

                if only_range < 1 {
                    println!("Your paswords can't be 0 length");
                    exit(0);
                }
                if only_range > 6 {
                    println!("Maximum length is 6!");
                    exit(0);
                }
            }
        } else {
            println!("Invalid argument!");
            exit(0);
        }
    }

    fn create_list(
        range_1: i8,
        only_range: i8,
        counter: i8,
        character_array: &[char; 61],
        mut file: &File,
    ) -> u64 {
        let mut sixth_character_array_index: usize = 0;
        let mut fifth_character_array_index: usize = 0;
        let mut fourth_character_array_index: usize = 0;
        let mut second_character_array_index: usize = 0;
        let mut third_character_array_index: usize = 0;
        let mut password_begin_index: usize = 0;
        let mut password_begin: String = character_array[password_begin_index].to_string();

        let mut password_length = 1;
        let error_message: &str = "Unable to write to text file!";
        let mut length_inner: u64 = 0;

        if range_1 > 0 {
            password_length = range_1;
        } else {
            ()
        }
        if only_range > 0 {
            password_length = only_range;
        } else {
            ()
        }
        if counter > 0 {
            password_length = counter;
        }

        if password_length == 1 {
            for character in character_array {
                let formated_string: String = format!("{}\n", character);
                let formated_in_bytes = formated_string.as_bytes();

                file.write(formated_in_bytes).expect(error_message);
                length_inner += 1;
            }
        } else if password_length == 2 {
            loop {
                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                length_inner += 1;

                second_character_array_index += 1;

                if second_character_array_index == character_array.len()
                    && password_begin != String::from("9")
                {
                    second_character_array_index = 0;
                    password_begin_index += 1;
                    password_begin = character_array[password_begin_index].to_string();
                }
                if password_begin == String::from("9")
                    && second_character_array_index == character_array.len()
                {
                    break;
                }
            }
        } else if password_length == 3 {
            loop {
                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                password.push(character_array[third_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                //Trigger if the first character is less then 9 -> <=8
                length_inner += 1;
                //First switch
                if password_begin_index != character_array.len() - 1 {
                    password_begin_index += 1;
                    password_begin = character_array[password_begin_index].to_string();
                }
                //Second switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    second_character_array_index += 1;
                }
                //Third switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index != 60
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    third_character_array_index += 1;
                }
                //Break
                else {
                    break;
                }
            }
        } else if password_length == 4 {
            loop {
                unsafe {
                    if BOOL_EXIT == true {
                        break;
                    }
                }

                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                password.push(character_array[third_character_array_index]);
                password.push(character_array[fourth_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                length_inner += 1;
                //First switch
                if password_begin_index != character_array.len() - 1 {
                    password_begin_index += 1;
                    password_begin = character_array[password_begin_index].to_string();
                }
                //Second switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    second_character_array_index += 1;
                }
                //Third switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    third_character_array_index += 1;
                }
                // Fourth switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index == character_array.len() - 1
                    && fourth_character_array_index != 60
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    third_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    fourth_character_array_index += 1;
                }
                //Break
                else {
                    break;
                }
            }
        } else if password_length == 5 {
            loop {
                unsafe {
                    if BOOL_EXIT == true {
                        break;
                    }
                }

                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                password.push(character_array[third_character_array_index]);
                password.push(character_array[fourth_character_array_index]);
                password.push(character_array[fifth_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                length_inner += 1;
                //First switch
                if password_begin_index != character_array.len() - 1 {
                    password_begin_index += 1;
                    password_begin = character_array[password_begin_index].to_string();
                }
                //Second switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    second_character_array_index += 1;
                }
                //Third switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    third_character_array_index += 1;
                }
                // Fourth switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index == character_array.len() - 1
                    && fourth_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    third_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    fourth_character_array_index += 1;
                }
                //Fifth switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index == character_array.len() - 1
                    && fourth_character_array_index == character_array.len() - 1
                    && fifth_character_array_index != 60
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    third_character_array_index = 0;
                    fourth_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    fifth_character_array_index += 1;
                }
                //Break
                else {
                    break;
                }
            }
        } else if password_length == 6 {
            loop {
                unsafe {
                    if BOOL_EXIT == true {
                        break;
                    }
                }

                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                password.push(character_array[third_character_array_index]);
                password.push(character_array[fourth_character_array_index]);
                password.push(character_array[fifth_character_array_index]);
                password.push(character_array[sixth_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                length_inner += 1;
                //First switch
                if password_begin_index != character_array.len() - 1 {
                    password_begin_index += 1;
                    password_begin = character_array[password_begin_index].to_string();
                }
                //Second switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    second_character_array_index += 1;
                }
                //Third switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    third_character_array_index += 1;
                }
                // Fourth switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index == character_array.len() - 1
                    && fourth_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    third_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    fourth_character_array_index += 1;
                }
                //Fifth switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index == character_array.len() - 1
                    && fourth_character_array_index == character_array.len() - 1
                    && fifth_character_array_index != character_array.len() - 1
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    third_character_array_index = 0;
                    fourth_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    fifth_character_array_index += 1;
                }
                //Sixth switch
                else if password_begin_index == character_array.len() - 1
                    && second_character_array_index == character_array.len() - 1
                    && third_character_array_index == character_array.len() - 1
                    && fourth_character_array_index == character_array.len() - 1
                    && fifth_character_array_index == character_array.len() - 1
                    && sixth_character_array_index != 60
                {
                    password_begin_index = 0;
                    second_character_array_index = 0;
                    third_character_array_index = 0;
                    fourth_character_array_index = 0;
                    password_begin = character_array[password_begin_index].to_string();
                    fifth_character_array_index = 0;
                    sixth_character_array_index += 1;
                }
                //Break
                else {
                    break;
                }
            }
        } else {
            println!("Passoword length over 6 is not supported yet!");
        }
        unsafe {
            if BOOL_EXIT == true {
                println!(
                    "{} length passwords aren't fully generated!",
                    password_length
                );
            } else {
                println!("{} length passwords are generated!", password_length);
            }
        }

        length_inner
    }

    if argument.contains("-range") {
        for _i in range_1..=range_2 {
            length += create_list(range_1, only_range, 0, &character_array, &file);

            range_1 += 1;
        }
    } else if argument.contains("-only") {
        length += create_list(0, only_range, 0, &character_array, &file);
    } else {
        let mut counter = 0;
        unsafe {
            while BOOL_EXIT == false || counter > 6 {
                counter += 1;
                length += create_list(0, 0, counter, &character_array, &file);
            }
        }
    }

    let length_string = length.to_string();

    let mut byte_array = length_string.as_bytes().to_vec();
    let mut index = byte_array.len() - 1;

    let mut index_int: i32 = index.try_into().unwrap();
    if index_int - 2 >= 0 {
        index -= 2;
        byte_array.insert(index, 32);
    }
    loop {
        index_int = index.try_into().unwrap();
        if index_int - 3 >= 0 {
            index -= 3;
            byte_array.insert(index, 32);
        } else {
            break;
        }
    }

    let byte_array_reverse = str::from_utf8(&byte_array).unwrap();

    println!(
        "Total number of generated passwords ---> |{}|",
        byte_array_reverse
    );
}

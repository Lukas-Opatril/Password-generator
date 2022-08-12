use std::fs::File;
use std::io::Write;

fn main() {
    println!("Rainbow table basic generator");
    println!("To abort program, please press Ctrl + C in Your console.");

    let mut password_length: i8 = 1;
    let error_message: &str = "Unable to write to text file!";
    let mut file = File::create("list.txt").expect("Failed to create a text file!");
    let character_array = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2',
        '3', '4', '5', '6', '7', '8', '9',
    ];
    let mut length: u64 = 0;
    let mut counter: u8 = 0;

    for character in character_array {
        let formated_string: String = format!("{}\n", character);
        let formated_in_bytes = formated_string.as_bytes();

        file.write(formated_in_bytes).expect(error_message);
        length += 1;
    }
    counter += 1;
    password_length += 1;
    println!("Passwords with length {} are generated!", counter);

    let mut create_list = || {
        if password_length == 2 {
            let mut character_array_index: usize = 0;
            let mut password_begin_index: usize = 0;
            let mut password_begin: String = character_array[password_begin_index].to_string();
            loop {
                let mut password = password_begin.clone();
                password.push(character_array[character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                length += 1;

                character_array_index += 1;

                if character_array_index == character_array.len()
                    && password_begin != String::from("9")
                {
                    character_array_index = 0;
                    password_begin_index += 1;
                    password_begin = character_array[password_begin_index].to_string();
                }
                if password_begin == String::from("9")
                    && character_array_index == character_array.len()
                {
                    break;
                }
            }
            counter += 1;
            println!("Passwords with length {} are generated!", counter);
        } else if password_length == 3 {
            let mut second_character_array_index: usize = 0;
            let mut third_character_array_index: usize = 0;
            let mut password_begin_index: usize = 0;
            let mut password_begin: String = character_array[password_begin_index].to_string();

            loop {
                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                password.push(character_array[third_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                //Trigger if the first character is less then 9 -> <=8
                length += 1;
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
            counter += 1;
            println!("Passwords with length {} are generated!", counter);
        } else if password_length == 4 {
            let mut fourth_character_array_index: usize = 0;
            let mut second_character_array_index: usize = 0;
            let mut third_character_array_index: usize = 0;
            let mut password_begin_index: usize = 0;
            let mut password_begin: String = character_array[password_begin_index].to_string();

            loop {
                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                password.push(character_array[third_character_array_index]);
                password.push(character_array[fourth_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                length += 1;
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
            counter += 1;
            println!("Passwords with length {} are generated!", counter);
        } else if password_length == 5 {
            let mut fifth_character_array_index: usize = 0;
            let mut fourth_character_array_index: usize = 0;
            let mut second_character_array_index: usize = 0;
            let mut third_character_array_index: usize = 0;
            let mut password_begin_index: usize = 0;
            let mut password_begin: String = character_array[password_begin_index].to_string();

            loop {
                let mut password = password_begin.clone();
                password.push(character_array[second_character_array_index]);
                password.push(character_array[third_character_array_index]);
                password.push(character_array[fourth_character_array_index]);
                password.push(character_array[fifth_character_array_index]);
                let formated_password = format!("{password}\n");
                file.write(formated_password.as_bytes())
                    .expect(error_message);
                length += 1;
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
            counter += 1;
            println!("Passwords with length {} are generated!", counter);
        }
        password_length += 1;
    };
    //Four cycles

    for _i in 1..5 {
        create_list();
    }

    println!("Total number of written passwords is : {}", length);
}

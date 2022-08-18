# Password-generator
## What it does?
The program creates all possible variations of passwords in a given length and then writes them in the 'list.txt' file or you can generate one random password in given length.
## How to run?
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. In the project folder run `cargo build --release`
3. `cd target/release`
4. `./password-generator`

### Supported arguments
1. -range ( number..number2 )
- You can specify the password length range of generated passwords.
- Example: `./password-generator -range 1 4` generates all possible variations between given range, including the second number.
2. -only ( number )
- Generates *only* passwords in a given length
- Example : `./password-generator -only 3`
3. -one (number )
- Generates *only* __one__ random password in a given length
- Example : `./password-generator -one 12`
- You can generate single passwords up to 15 characters.

### Additional information
 __Max password length is 6 because to generate *,142742836×10¹²* passwords (61^7) you would need a *really powerful machine to finish the program in hours, not in days or years.*__
 
 __If you want to generate all possible variations of 5 or 6-length passwords, make sure, you have enough disk drive space :) 10GB+__
 
 __You cant generate zero-length passwords.__
 
 ***If you execute the program with just `./password-generator`, it will generate passwords until it generates all passwords in range 1--6 or until you stop it with Ctrl+C or something***


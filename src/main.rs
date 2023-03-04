use std::io;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use rand::{thread_rng, Rng};
use std::process::Command;
use regex::Regex;

/// .
fn fully_mutate_content(buffer: &mut Vec<u8>) {

    let mut rng = thread_rng();
    //println!("[+] Original content: {:?}", buffer);

    for i in 0..buffer.len() {
        let random_number = rng.gen_range(0..=255);
        buffer[i] = random_number;
    }

    //println!("[+] Modified content: {:?}", buffer);

    
}

fn one_byte_mutate_content(buffer: &mut Vec<u8>) {
    let mut rng = thread_rng();
    let random_number = rng.gen_range(0..buffer.len());
    let random_number2 = rng.gen_range(0..=255);
    buffer[random_number] = random_number2;
    
}

fn range_zero_byte_mutate_content(buffer: &mut Vec<u8>) {
    let mut rng = thread_rng();
    let interval = rng.gen_range(0..20);
    //println!("[+] Number of bytes to mutate: {}", interval);
    let zero_byte = 0;
    let start = rng.gen_range(0..buffer.len());
    //println!("[+] Start index: {}", start);
    let mut end = {
        if start + interval > buffer.len() {
            buffer.len()
        }
        else{
            start+interval
        }
    };
    //println!("[+] End index: {}", end);
    for i in start..end {
        buffer[i] = zero_byte;
    }
}

fn main() -> std::io::Result<()> {
    
    println!("Enter the file path: ");
    let mut user_input = String::new();
    //io::stdin().read_line(&mut user_input).expect("Failed to read user input");
    user_input = String::from("/home/m4st3rm1nd/Downloads/cross.jpg");
    println!("[+] Your file path is: {}", user_input);
    let mut file = File::open(user_input.trim()).expect("Failed to open file");

    let metadata = std::fs::metadata(user_input.trim()).expect("Failed to get file metadata");
    let size = metadata.len();
    println!("[+] File size: {}", size);

    let mut buffer = vec![0; size as usize];
    file.read_exact(&mut buffer).expect("Failed to read file");
    
    //println!("[+] File content: {:?}", buffer);
    //println!("[+] File content as string: {:?}", String::from_utf8_lossy(&buffer));

    // for element in buffer.iter_mut(){
    //     *element = 0;
    // }
    //println!("[+] After 0 out File content: {:?}", buffer);
    //let new_file = String::from("/home/m4st3rm1nd/Documents/test2.jpg");
    
    //println!("[+] Writing modified file to a new file: {}", new_file);
    //fs::write(new_file, &buffer).expect("Failed to write file");

    let another_file = String::from("/home/m4st3rm1nd/Documents/test3.jpg");
    let mut cmd_another_file = another_file.clone();
    let mut test_another_file = String::from("/home/m4st3rm1nd/Documents/test3.jpg");
    let mut clone_test_another_file = test_another_file.clone();
    let mut stderr_vector: Vec<String> = Vec::new();
    let mut stderr_element = String::new();
    // run the fuzzer
    for _i in 0..20000{
        //println!("[+] vector content at the beginning: {:?}", stderr_vector);
        //fully_mutate_content(&mut buffer);
        //one_byte_mutate_content(&mut buffer);
        range_zero_byte_mutate_content(&mut buffer);
        fs::write(&mut clone_test_another_file, &buffer).expect("Failed to write file");

        let output = Command::new("/home/m4st3rm1nd/Downloads/jpeg2bmp")
            //.arg("/home/m4st3rm1nd/Downloads/jpeg2bmp")
            .arg(&mut cmd_another_file)
            .arg("test3.bmp")
            .output()
            .expect("failed to execute process");

        
        //println!("status: {}", output.status);
        //println!("stdout: {}",String::from_utf8_lossy(&output.stdout));

        let cmd_std_err = String::from_utf8_lossy(&output.stderr);
        //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        stderr_vector.push(cmd_std_err.to_string());
        //println!("cmd_std_err.to_string(): {}", cmd_std_err.to_string());
        //println!("[+] vector content at the middle: {:?}", stderr_vector);

        if cmd_std_err.to_string().contains("Triggering Bug"){
            let re = Regex::new(r"#(\d+)").unwrap();
            let mut bug_number = String::new();
            if let Some(captures) = re.captures(&cmd_std_err.to_string()) {
                bug_number = captures[1].to_string();
                //println!("Found bug number: {}", bug_number);
            }
            let filename_start = "/home/m4st3rm1nd/Documents/bugType";
            let filename_middle = filename_start.to_string() + &bug_number;
            let filename_full = filename_middle.to_string() + ".jpg";
            //println!("[+] Filename full: {}", filename_full);
            fs::rename(&mut test_another_file, filename_full).expect("Failed to write the file");
        //     if !stderr_vector.contains(&cmd_std_err.to_string()) {
        //         //println!("[+] Triggering Bug");
        //         println!("[+] Triggering Bug");
        //         println!("[+] Bug: {}", String::from_utf8_lossy(&output.stderr));
        //         //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        //     }
        //     else{
        //         println!("[+++] Triggering Bug");
        //     }
            
        }
        // else {
        //     continue;
        // }
    }
    stderr_vector.sort();
    stderr_vector.dedup();
    println!("[+] Final vector content: {:?}", stderr_vector);
    
    
    
    Ok(())
}

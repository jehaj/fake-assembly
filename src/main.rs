use std::collections::HashMap;
use std::env::args;
use std::ffi::OsString;
use std::fs::read_to_string;
use std::fs::read_dir;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let args = args();
    let file_path = match args.skip(1).next() {
        None => panic!("Path to assembly file was not provided. Use 'fake-assembly <file>'."),
        Some(path) => path
    };
    let file_path = Path::new(&file_path);
    let program = match read_to_string(file_path) {
        Err(err) => panic!("Could not read file. Error is {err}"),
        Ok(s) => s,
    };
    let program = program.lines().filter(|&l| l.trim() != "");
    let mut program_without_labels = Vec::new();
    let mut label_lookup = HashMap::new();
    let mut counter = 0;
    for (line_no, line) in program.enumerate() {
        if line.contains(':') {
            // line contains a label
            let mut line_split = line.split(':');
            let label = line_split.next().unwrap().trim();
            if label.len() == 0 { panic!("Empty label at line {}", line_no+1) }
            let command = line_split.next().unwrap().trim();
            let command_on_same_line = command.trim().len() != 0;
            label_lookup.insert(label, counter + !command_on_same_line as usize);
            if !command_on_same_line { continue; }
        }
        let command = line.trim();
        program_without_labels.push(command);
        counter += 1;
    }
    println!("Your program has been registered as");
    for &line in &program_without_labels {
        println!("{line}");
    }
    println!("\nRunning your program...");
    let mut registers = [0i32; 8];
    let mut program_counter = 0;
    let mut zero_flag: bool = true;
    while program_counter < program_without_labels.len() {
        let instruction = program_without_labels[program_counter];
        let (command, operands) = instruction.split_once(' ').expect("Expected operand...");
        let mut operands = operands.split(',').map(|s| s.trim());
        let first_operand = operands.next();
        let second_operand = operands.next();
        let third_operand = operands.next();
        match command.to_uppercase().as_str() {
            "ZERO" => {
                let register = first_operand.expect("Expected register to zero");
                let no = get_register_no(register, 1);
                registers[no] = 0;
            },
            "MOV" => {
                let dst_register = first_operand.expect("Expected destination register");
                let src_register = second_operand.expect("Expected source register");
                let dst_register = get_register_no(dst_register, 1);
                let src_register = get_register_no(src_register, 2);
                registers[dst_register] = registers[src_register];
            },
            "ADD" => {
                let a_register = first_operand.expect("Expected destination register");
                let b_register = second_operand.expect("Expected source register");
                let c_register = second_operand.expect("Expected source register");
                let a_register = get_register_no(a_register, 1);
                let b_register = get_register_no(b_register, 2);
                let c_register = get_register_no(c_register, 3);
                registers[a_register] = registers[b_register] + registers[c_register];
                zero_flag = registers[a_register] == 0;
            },
            "SUB" => {
                let a_register = first_operand.expect("Expected destination register");
                let b_register = second_operand.expect("Expected source register");
                let c_register = second_operand.expect("Expected source register");
                let a_register = get_register_no(a_register, 1);
                let b_register = get_register_no(b_register, 2);
                let c_register = get_register_no(c_register, 3);
                registers[a_register] = registers[b_register] - registers[c_register];
                zero_flag = registers[a_register] == 0;
            },
            "INC" => {
                let register = first_operand.expect("Expected register to increment");
                let no = get_register_no(register, 1);
                registers[no] += 1;
                zero_flag = registers[no] == 0;
            },
            "DEC" => {
                let register = first_operand.expect("Expected register to increment");
                let no = get_register_no(register, 1);
                registers[no] -= 1;
                zero_flag = registers[no] == 0;
            },
            "AND" => {
                let a_register = first_operand.expect("Expected destination register");
                let b_register = second_operand.expect("Expected source register");
                let c_register = second_operand.expect("Expected source register");
                let a_register = get_register_no(a_register, 1);
                let b_register = get_register_no(b_register, 2);
                let c_register = get_register_no(c_register, 3);
                registers[a_register] = registers[b_register] & registers[c_register];
                zero_flag = registers[a_register] == 0;
            },
            "OR" => {
                let a_register = first_operand.expect("Expected destination register");
                let b_register = second_operand.expect("Expected source register");
                let c_register = second_operand.expect("Expected source register");
                let a_register = get_register_no(a_register, 1);
                let b_register = get_register_no(b_register, 2);
                let c_register = get_register_no(c_register, 3);
                registers[a_register] = registers[b_register] | registers[c_register];
                zero_flag = registers[a_register] == 0;
            },
            "XOR" => {
                let a_register = first_operand.expect("Expected destination register");
                let b_register = second_operand.expect("Expected source register");
                let c_register = second_operand.expect("Expected source register");
                let a_register = get_register_no(a_register, 1);
                let b_register = get_register_no(b_register, 2);
                let c_register = get_register_no(c_register, 3);
                registers[a_register] = registers[b_register] ^ registers[c_register];
                zero_flag = registers[a_register] == 0;
            },
            "NOT" => {
                let register = first_operand.expect("Expected register to not");
                let no = get_register_no(register, 1);
                registers[no] = !registers[no];
                zero_flag = registers[a_register] == 0;
            },
            "SHL" => {
                let register = first_operand.expect("Expected register to shift left");
                let amount = second_operand.expect("Expected amount to shift");
                let no = get_register_no(register, 1);
                let amount = amount.chars().next().unwrap().to_digit(10).expect("Not a number...");
                registers[no] = registers[no] << amount;
            },
            "SHR" => {
                let register = first_operand.expect("Expected register to shift right");
                let amount = second_operand.expect("Expected amount to shift");
                let no = get_register_no(register, 1);
                let amount = amount.chars().next().unwrap().to_digit(10).expect("Not a number...");
                registers[no] = registers[no] >> amount;
            },
            "JZ" => {
                if zero_flag == false {
                    let label = first_operand.expect("Expected label to jump to");
                    program_counter = match label_lookup.get(label) {
                        None => { panic!("Could not find label") },
                        Some(&v) => v
                    };
                    continue
                }
            },
            "JNZ" => {
                if zero_flag != false {
                    let label = first_operand.expect("Expected label to jump to");
                    program_counter = match label_lookup.get(label) {
                        None => { panic!("Could not find label") },
                        Some(&v) => v
                    };
                    continue
                }
            },
            "J" => {
                let label = first_operand.expect("Expected label to jump to");
                program_counter = match label_lookup.get(label) {
                    None => { panic!("Could not find label") },
                    Some(&v) => v
                };
                continue
            },
            _ => { panic!("The command '{command}' is not recognized. See README for instruction set.")}
        };
        program_counter += 1;
    }
    println!("Your program is done now.");
    println!("The registers contain");
    println!("{}", registers.iter().enumerate().map(
        |(i, &x)| format!("R{i}={x}")
    ).collect::<Vec<String>>()
        .join(", "));
    println!("{}", registers.iter().enumerate().map(
        |(i, &x)| format!("R{i}=0b{x:04b}")
    ).collect::<Vec<String>>()
        .join(", "));
    println!("Exiting...");
}

fn get_register_no(register: &str, i: usize) -> usize {
    if register.len() != i+1 { panic!("Register is formatted wrongly") }
    let second_char = register.chars().nth(i).unwrap();
    let no = second_char.to_digit(10).expect("Expected a register with a number");
    no as usize
}

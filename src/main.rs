use std::collections::VecDeque;
use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use libc::{chdir, execvp, fork, getcwd, mkdir, rmdir, waitpid, WIFEXITED, WEXITSTATUS, PATH_MAX};

fn main() {
    let mut directory_stack: VecDeque<String> = VecDeque::new();

    loop {
        // Display prompt with the current directory
        let cwd = current_directory().unwrap_or_else(|_| "unknown".into());
        print!("{} > ", cwd);
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Parse command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "pwd" => print_current_directory(),
            "cd" => change_directory(args),
            "pushd" => push_directory(args, &mut directory_stack),
            "popd" => pop_directory(&mut directory_stack),
            "dirs" => print_directories(&directory_stack),
            "echo" => echo_command(args),
            "clear" => clear_screen(),
            "ls" => list_directory(args),
            "mkdir" => make_directory(args),
            "rmdir" => remove_directory(args),
            "exit" => break,
            cmd => execute_basic_command(cmd, args),
        }
    }
}

fn current_directory() -> Result<String, String> {
    let mut buf = vec![0u8; PATH_MAX as usize];
    unsafe {
        if getcwd(buf.as_mut_ptr() as *mut i8, PATH_MAX).is_null() {
            Err("Failed to get current directory".to_string())
        } else {
            Ok(CString::from_raw(buf.as_mut_ptr() as *mut i8).into_string().unwrap())
        }
    }
}

fn print_current_directory() {
    match current_directory() {
        Ok(dir) => println!("{}", dir),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn change_directory(args: &[&str]) {
    let new_dir = args.get(0).map_or("/", |&dir| dir);
    let c_new_dir = CString::new(new_dir).unwrap();
    unsafe {
        if chdir(c_new_dir.as_ptr()) != 0 {
            eprintln!("cd: Failed to change directory to {}", new_dir);
        }
    }
}

fn push_directory(args: &[&str], stack: &mut VecDeque<String>) {
    if let Ok(current_dir) = current_directory() {
        stack.push_front(current_dir);
        change_directory(args);
    } else {
        eprintln!("pushd: Unable to get current directory");
    }
}

fn pop_directory(stack: &mut VecDeque<String>) {
    if let Some(dir) = stack.pop_front() {
        let c_dir = CString::new(dir.clone()).unwrap();
        unsafe {
            if chdir(c_dir.as_ptr()) != 0 {
                eprintln!("popd: Failed to change directory to {}", dir);
            }
        }
    } else {
        eprintln!("popd: Directory stack is empty");
    }
}

fn print_directories(stack: &VecDeque<String>) {
    if stack.is_empty() {
        println!("Directory stack is empty");
    } else {
        for dir in stack {
            println!("{}", dir);
        }
    }
}

fn echo_command(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

fn list_directory(_args: &[&str]) {
    execute_basic_command("ls", &[]);
}

fn make_directory(args: &[&str]) {
    if let Some(dir_name) = args.get(0) {
        let c_dir_name = CString::new(*dir_name).unwrap();
        unsafe {
            if mkdir(c_dir_name.as_ptr(), 0o755) != 0 {
                eprintln!("mkdir: Failed to create directory {}");
            }
        }
    } else {
        eprintln!("mkdir: Missing directory name");
    }
}

fn remove_directory(args: &[&str]) {
    if let Some(dir_name) = args.get(0) {
        let c_dir_name = CString::new(*dir_name).unwrap();
        unsafe {
            if rmdir(c_dir_name.as_ptr()) != 0 {
                eprintln!("rmdir: Failed to remove directory {}");
            }
        }
    } else {
        eprintln!("rmdir: Missing directory name");
    }
}

fn execute_basic_command(cmd: &str, args: &[&str]) {
    unsafe {
        let pid = fork();
        if pid < 0 {
            eprintln!("Failed to fork process");
        } else if pid == 0 {
            // Child process: Execute command
            let c_cmd = CString::new(cmd).unwrap();
            let c_args: Vec<CString> = args.iter().map(|&arg| CString::new(arg).unwrap()).collect();
            let mut c_args_ptrs: Vec<*const i8> = c_args.iter().map(|arg| arg.as_ptr()).collect();
            c_args_ptrs.insert(0, c_cmd.as_ptr());
            c_args_ptrs.push(std::ptr::null());

            execvp(c_cmd.as_ptr(), c_args_ptrs.as_ptr());
            eprintln!("Command not found: {}", cmd);
            libc::_exit(1);
        } else {
            // Parent process: Wait for child to complete
            let mut status = 0;
            waitpid(pid, &mut status, 0);
            if WIFEXITED(status) {
                println!("Exit status: {}", WEXITSTATUS(status));
            }
        }
    }
}

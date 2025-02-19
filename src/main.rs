use nix::unistd::Uid;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, write};
use std::io::{self, Write};
use std::ffi::CString;

fn main() -> io::Result<()> {

    let uid = Uid::current().as_raw() as u32;
    println!("Usuario: {:?}", uid);


    let path = CString::new("/tmp/notes").expect("CString conversion failed");
    let path_str = path.as_c_str();


    let fd = match open(
        path_str,
        OFlag::O_CREAT | OFlag::O_APPEND | OFlag::O_WRONLY,
        Mode::S_IRUSR | Mode::S_IWUSR,
    ) {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to open file"));
        }
    };


    match write(fd, &uid.to_ne_bytes()) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to write UID: {}", e);
            close(fd).ok();
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to write UID"));
        }
    }


    match write(fd, b"\n") {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to write newline: {}", e);
            close(fd).ok();
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to write newline"));
        }
    }


    println!("Enter your message:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;


    match write(fd, input.as_bytes()) {
        Ok(_) => println!("Your message has been written to /tmp/notes."),
        Err(e) => {
            eprintln!("Failed to write message: {}", e);
            close(fd).ok();
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to write message"));
        }
    }


    if let Err(e) = close(fd) {
        eprintln!("Failed to close file: {}", e);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to close file"));
    }

    Ok(())
}

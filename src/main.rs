use nix::unistd::{Uid, write};
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use std::fs::{File, OpenOptions};

fn main() -> io::Result<()> {
    // Get the current user's UID
    let uid = Uid::current().as_raw();

    // Convert UID to 4 bytes in little-endian format
    let uid_bytes: [u8; 4] = uid.to_le_bytes();

    // Open or create the file in /tmp/notes using file descriptors
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/notes")?;

    let fd = file.as_raw_fd();

    // Write UID to file using file descriptor
    nix::unistd::write(fd, &uid_bytes)?;

    // Write a newline after UID
    nix::unistd::write(fd, b"\n")?;

    // Read user input
    println!("Enter your message:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Write the user's message to the file using file descriptor
    nix::unistd::write(fd, input.as_bytes())?;

    println!("Your message has been written to /tmp/notes.");

    Ok(())
}
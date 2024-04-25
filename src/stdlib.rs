use crate::util::*;
use crate::error::*;

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::io::{BufRead, Write};

type LibFunction = fn(Vec<RickrollObject>, &mut dyn Write, &mut dyn BufRead) -> Result<RickrollObject, Error>;

lazy_static! {
    pub static ref BUILTIN_FUNCTIONS: HashMap<String, LibFunction> = {
        let mut m = HashMap::new();
        m.insert(String::from("ArrayOf"), array_of as LibFunction);
        m.insert(String::from("ArrayPop"), array_pop as LibFunction);
        m.insert(String::from("ArrayPush"), array_push as LibFunction);
        m.insert(String::from("ArrayReplace"), array_replace as LibFunction);
        m.insert(String::from("ArrayLength"), array_length as LibFunction);
        m.insert(String::from("PutChar"), put_char as LibFunction);
        m.insert(String::from("ReadLine"), read_line as LibFunction);
        m
    };
}

fn array_of(args: Vec<RickrollObject>, _: &mut dyn Write, _: &mut dyn BufRead) -> Result<RickrollObject, Error> {
    return Ok(RickrollObject::Array(args));
}

fn array_pop(args: Vec<RickrollObject>, _: &mut dyn Write, _: &mut dyn BufRead) -> Result<RickrollObject, Error> {
    if args.len() != 2 {
        return Err(Error::new(ErrorType::RuntimeError, "Wrong number of arguments for ArrayPop", None));
    }
    let arr = args[0].clone();
    let idx = args[1].clone();
    if let RickrollObject::Array(mut x) = arr {
        if let RickrollObject::Int(y) = idx {
            if y >= 0 && (y as usize) < x.len() {
                x.remove(y as usize);
                return Ok(RickrollObject::Array(x));
            }  else {
                return Err(Error::new(ErrorType::RuntimeError, "Array Index out of Bounds", None));
            }
        }
    }
    return Err(Error::new(ErrorType::RuntimeError, "Wrong type of arguments for ArrayPop", None));
}

fn array_push(args: Vec<RickrollObject>, _: &mut dyn Write, _: &mut dyn BufRead) -> Result<RickrollObject, Error> {
    if args.len() != 3 {
        return Err(Error::new(ErrorType::RuntimeError, "Wrong number of arguments for ArrayPush", None));
    }
    let arr = args[0].clone();
    let idx = args[1].clone();
    let val = args[2].clone();
    if let RickrollObject::Array(mut x) = arr {
        if let RickrollObject::Int(y) = idx {
            if y >= 0 && (y as usize) <= x.len() {
                x.insert(y as usize, val);
                return Ok(RickrollObject::Array(x));
            } else {
                return Err(Error::new(ErrorType::RuntimeError, "Array Index out of Bounds", None));
            }
        }
    }
    return Err(Error::new(ErrorType::RuntimeError, "Wrong type of arguments for ArrayPush", None));
}

fn array_replace(args: Vec<RickrollObject>, _: &mut dyn Write, _: &mut dyn BufRead) -> Result<RickrollObject, Error> {
    if args.len() != 3 {
        return Err(Error::new(ErrorType::RuntimeError, "Wrong number of arguments for ArrayReplace", None));
    }
    let arr = args[0].clone();
    let idx = args[1].clone();
    let val = args[2].clone();
    if let RickrollObject::Array(mut x) = arr {
        if let RickrollObject::Int(y) = idx {
            if y >= 0 && (y as usize) < x.len() {
                x[y as usize] = val;
                return Ok(RickrollObject::Array(x));
            } else {
                return Err(Error::new(ErrorType::RuntimeError, "Array Index out of Bounds", None));
            }
        }
    }
    return Err(Error::new(ErrorType::RuntimeError, "Wrong type of arguments for ArrayReplace", None));
}

fn array_length(args: Vec<RickrollObject>, _: &mut dyn Write, _: &mut dyn BufRead) -> Result<RickrollObject, Error> {
    if args.len() != 1 {
        return Err(Error::new(ErrorType::RuntimeError, "Wrong number of arguments for ArrayLength", None));
    }
    let arr = args[0].clone();
    if let RickrollObject::Array(x) = arr {
        return Ok(RickrollObject::Int(x.len() as i32));
    }
    return Err(Error::new(ErrorType::RuntimeError, "Wrong type of arguments for ArrayLength", None));
}

fn put_char(args: Vec<RickrollObject>, writer: &mut dyn Write, _: &mut dyn BufRead) -> Result<RickrollObject, Error> {
    if args.len() != 1 {
        return Err(Error::new(ErrorType::RuntimeError, "Wrong number of arguments for PutChar", None));
    }
    let chr = args[0].clone();
    if let RickrollObject::Char(x) = chr {
        write!(writer, "{}", x).unwrap();
        return Ok(RickrollObject::Undefined);
    }
    return Err(Error::new(ErrorType::RuntimeError, "Wrong type of arguments for PutChar", None));
}

fn read_line(args: Vec<RickrollObject>, _: &mut dyn Write, reader: &mut dyn BufRead) -> Result<RickrollObject, Error> {
    if args.len() != 0 {
        return Err(Error::new(ErrorType::RuntimeError, "Wrong number of arguments for ReadLine", None));
    }
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut arr = Vec::new();
    for c in line.chars() {
        if c == '\n' || c == '\r' {
            continue;
        }
        arr.push(RickrollObject::Char(c));
    }
    return Ok(RickrollObject::Array(arr));
}

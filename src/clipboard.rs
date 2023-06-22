use std::{io::Error, ptr};

use scopeguard::defer;
use winapi::{
    shared::minwindef::FALSE,
    um::{
        winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE},
        winuser::{CloseClipboard, OpenClipboard, SetClipboardData, CF_UNICODETEXT},
    },
};

pub fn copy_to_clipboard(text: &str, iteration: Option<u8>) -> Result<(), Error> {
    // magic for me : https://stackoverflow.com/questions/61981453/working-with-winapi-setclipboarddata-in-rust-language
    println!("trying to copy ({}) to clipboard", text);
    let iteration_number = if iteration.is_some() {
        iteration.unwrap()
    } else {
        0
    };
    let mut text_utf16: Vec<u16> = text.encode_utf16().collect();
    text_utf16.push(0);

    let hglob: *mut winapi::ctypes::c_void =
        unsafe { GlobalAlloc(GMEM_MOVEABLE, text_utf16.len() * std::mem::size_of::<u16>()) };
    if hglob == ptr::null_mut() {
        println!("Error creating hglob");
        return Err(Error::last_os_error());
    }
    defer!(unsafe { GlobalFree(hglob) };);

    let dst = unsafe { GlobalLock(hglob) };
    if dst == ptr::null_mut() {
        println!("Error creating dst");
        return Err(Error::last_os_error());
    }

    unsafe {
        ptr::copy_nonoverlapping(text_utf16.as_ptr(), dst as _, text_utf16.len());
    }
    unsafe { GlobalUnlock(hglob) };

    let sucess = unsafe { OpenClipboard(ptr::null_mut()) } != FALSE;
    if !sucess {
        return Err(Error::last_os_error());
    }

    defer!(unsafe {CloseClipboard()};);
    let success = unsafe { SetClipboardData(CF_UNICODETEXT, hglob) } != ptr::null_mut();
    if !success {
        // TODO: Every 2 iterations, this fails, fix it somehow?
        if iteration_number > 1 {
            println!("Error opening clipboard");
            return Err(Error::last_os_error());
        }

        return copy_to_clipboard(text, Some(iteration_number + 1));
    }

    Ok(())
}

const SUBKEY_PATH_NOTEPAD: &str = r"SOFTWARE\Microsoft\Notepad";
const VALUE_NAME_DEFAULT_ENCODING: &str = r"iDefaultEncoding";
const VALUE_NAME_UNIX_MODE: &str = r"fUnixMode";

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum NotepadEncoding {
    ANSI = 1,
    UTF16_LE,
    UTF16_BE,
    UTF8_BOM,
    UTF8,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum NotepadLineEndings {
    Windows_CRLF,
    UNIX_LF,
}

fn main() -> Result<(), registry::Error> {
    let notepad_encoding = NotepadEncoding::UTF8;
    let notepad_line_endings = NotepadLineEndings::UNIX_LF;

    {
        let notepad_subkey = registry::Hive::CurrentUser.open(SUBKEY_PATH_NOTEPAD, registry::Security::AllAccess)?;
        notepad_subkey.set_value(VALUE_NAME_DEFAULT_ENCODING, &registry::Data::U32(notepad_encoding as u32));
        notepad_subkey.set_value(VALUE_NAME_UNIX_MODE, &registry::Data::U32(notepad_line_endings as u32));
    }

    println!("Notepad settings of `new instance` have been successfully updated to => {:?}, {:?}", notepad_encoding, notepad_line_endings);
    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
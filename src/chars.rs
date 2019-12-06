#![crate_type = "staticlib"]

use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::str::Utf8Error;
//use unicode_width::UnicodeWidthChar;
use std::ptr;

// **CONSTANTS FROM NANO.H**
const DEL_CODE:i8 = 0x7f;
#[cfg(ENABLE_UTF8)]
const MAXCHARLEN:i8 = 6;
#[cfg(not(ENABLE_UTF8))]
const MAXCHARLEN:i8 = 1;
const HIGHEST_POSITIVE:usize = (!0) >> 1;

static mut use_utf8: bool = false;

#[no_mangle]
#[cfg(ENABLE_UTF8)]
/* Enable UTF-8 support. */
pub extern "C" fn utf8_init() {
    unsafe {
        use_utf8 = true;
    }
}

#[no_mangle]
#[cfg(ENABLE_UTF8)]
/* Is UTF-8 support enabled? */
pub extern "C" fn using_utf8() -> bool
{
    unsafe {
        use_utf8
    }
}

/* Concatenate two allocated strings, and free the second. */
// char *addstrings(char* str1, size_t len1, char* str2, size_t len2)
// {
// 	str1 = charealloc(str1, len1 + len2 + 1);
// 	str1[len1] = '\0';

// 	strncat(&str1[len1], str2, len2);
// 	free(str2);

// 	return str1;
// }

/* Return TRUE if the value of c is in byte range, and FALSE otherwise. */
#[no_mangle]
pub extern "C" fn is_byte(c: i32) -> bool {
    c == ((c as i8) as i32)
}

/* This function is equivalent to isalpha() for multibyte characters. */
#[no_mangle]
pub extern "C" fn is_alpha_mbchar(c: *const c_char) -> bool {
    let wc: char;
    unsafe
    {
        wc = CStr::from_ptr(c).to_str()
        .expect("Encountered invalid char in a call to is_alpha_mbchar")
        .chars().nth(0).unwrap();
    }
    wc.is_alphabetic()
}

/* This function is equivalent to isalnum() for multibyte characters. */
#[no_mangle]
pub extern "C" fn is_alnum_mbchar(c: *const c_char) -> bool {
    let wc: char;
    unsafe
    {
        wc = CStr::from_ptr(c).to_str()
        .expect("Encountered invalid char in a call to is_alnum_mbchar")
        .chars().nth(0).unwrap();
    }
    wc.is_alphanumeric()
}

/* This function is equivalent to isblank() for multibyte characters. */
#[no_mangle]
pub extern "C" fn is_blank_mbchar(c: *const c_char) -> bool {
    let wc: char;
    unsafe
    {
        wc = CStr::from_ptr(c).to_str()
        .expect("Encountered invalid char in a call to is_blank_mbchar")
        .chars().nth(0).unwrap();
    }
    wc.is_whitespace()
}

/* This function is equivalent to iscntrl(), except in that it only
 * handles non-high-bit control characters. */
#[no_mangle]
pub extern "C" fn is_ascii_cntrl_char(c: i32) -> bool
{
    (0 <= c && c < 32)
}

/* This function is equivalent to iscntrl() for multibyte characters,
 * except in that it also handles multibyte control characters with
 * their high bits set. */
 #[no_mangle]
pub unsafe extern "C" fn is_cntrl_mbchar(c: *const c_char) -> bool
{
    if cfg!(ENABLE_UTF8) && using_utf8() 
    {
        return (*c as u8 & 0xE0) == 0 || *c == 127 ||
 				(*c == -62 && *c.offset(1) < -96);
    }
    ((*c as u8 & 0x60) == 0 || *c as u8 == 127)
}


/* This function is equivalent to ispunct() for multibyte characters. */
#[no_mangle]
pub extern "C" fn is_punct_mbchar(c: *const c_char) -> bool {
    let wc: char;
    unsafe
    {
        wc = CStr::from_ptr(c).to_str()
        .expect("Encountered invalid char in a call to is_alpha_mbchar")
        .chars().nth(0).unwrap();
    }
    wc.is_ascii_punctuation()
}

/* Return TRUE when the given multibyte character c is a word-forming
 * character (that is: alphanumeric, or specified in wordchars, or
 * punctuation when allow_punct is TRUE), and FALSE otherwise. */
// bool is_word_mbchar(const char *c, bool allow_punct)
// {
// 	if (*c == '\0')
// 		return FALSE;

// 	if (is_alnum_mbchar(c))
// 		return TRUE;

// 	if (word_chars != NULL && *word_chars != '\0') {
// 		char symbol[MAXCHARLEN + 1];
// 		int symlen = parse_mbchar(c, symbol, NULL);

// 		symbol[symlen] = '\0';
// 		return (strstr(word_chars, symbol) != NULL);
// 	}

// 	return (allow_punct && is_punct_mbchar(c));
// }
/* #[no_mangle]
pub extern "C" fn is_word_mbchar(c: *const c_char, allow_punct: bool) -> bool
{
    if *c=='\0' as c_char {
        return false;
    }
    if is_alnum_mbchar(c) {
        return true;
    }
    if !word_chars.is_null() && *word_chars!=('\0' as c_char) {
        let symbol = Vec::with_capacity(MAXCHARLEN+1);
        let symlen;
    }
} */

/* Return the visible representation of control character c. */
#[no_mangle]
pub extern "C" fn control_rep(c: i8) -> i8 {
    let val: i8;
    if c == DEL_CODE {
        val = '?' as i8;
    }
    else if c == -97 {
        val = '=' as i8;
    }
    else if c < 0 {
        val =  (c as i16 + 224) as i8
    }
    else {
        val = c + 64
    }
    val
}

/* Return the visible representation of multibyte control character c. */
// char control_mbrep(const char *c, bool isdata)
// {
// 	/* An embedded newline is an encoded NUL if it is data. */
// 	if (*c == '\n' && (isdata || as_an_at))
// 		return '@';

// #ifdef ENABLE_UTF8
// 	if (use_utf8) {
// 		if ((unsigned char)c[0] < 128)
// 			return control_rep(c[0]);
// 		else
// 			return control_rep(c[1]);
// 	} else
// #endif
// 		return control_rep(*c);
// }

// #[no_mangle]
// pub extern "C" fn control_mbrep(c: *const c_char, isdata: bool) -> c_char {
//     /* An embedded newline is an encoded NUL if it is data*/
//     if *c==('\n' as c_char) && (isdata || as_an_at){
//         return '@' as c_char;
//     }
// }

fn mbtowc(c: *const c_char) -> Result<char, Utf8Error> {
    unsafe
    {
        match CStr::from_ptr(c).to_str() {
            Ok(v) => Ok(v.chars().nth(0).unwrap()),
            Err(e) => Err(e)
        }
    }
}

// int mbwidth(const char *c)
// {
// 	/* Ask for the width only when the character isn't plain ASCII. */
// 	if ((signed char)*c <= 0) {
// 		wchar_t wc;
// 		int width;

// 		if (mbtowc(&wc, c, MAXCHARLEN) < 0)
// 			return 1;

// 		width = wcwidth(wc);

// 		if (width < 0)
// 			return 1;

// 		return width;
// 	} else
// 		return 1;
// }
// #endif

// #[no_mangle]
// #[cfg(ENABLE_UTF8)]
// pub extern "C" fn mbwidth(c: *const c_char) -> isize {
//     /* Ask for the width only when the character isn't plain ASCII. */
// 	if *c <= 0 {
// 		let wc:char;
// 		let width:isize;

// 		match mbtowc(c) {
//             Ok(v) => wc = v,
//             Err(e) => return 1
//         }
        
//         match UnicodeWidthChar::width(wc) {
//             Some(v) => v,
//             None => 1
//         }
//     }
//     else {1}
// }

/* Convert the Unicode value in code to a multibyte character, if possible.
 * If the conversion succeeds, return the (dynamically allocated) multibyte
 * character and its length.  Otherwise, return an undefined (dynamically
 * allocated) multibyte character and a length of zero. */
// char *make_mbchar(long code, int *length)
// {
// 	char *mb_char;

// #ifdef ENABLE_UTF8
// 	if (use_utf8) {
// 		mb_char = charalloc(MAXCHARLEN);
// 		*length = wctomb(mb_char, (wchar_t)code);

// 		/* Reject invalid Unicode characters. */
// 		if (*length < 0 || !is_valid_unicode((wchar_t)code)) {
// 			IGNORE_CALL_RESULT(wctomb(NULL, 0));
// 			*length = 0;
// 		}
// 	} else
// #endif
// 	{
// 		mb_char = mallocstrncpy(NULL, (char *)&code, 1);
// 		*length = 1;
// 	}

// 	return mb_char;
// }
// #[no_mangle]
// pub extern "C" make_mbchar(code: i64, length: isize) -> *const c_char
// {

// }

//#[no_mangle]
/* Return the length (in bytes) of the character located at *pointer. */
pub extern "C" fn char_length(pointer:*const c_char) ->usize
{
    let wc:char;
	unsafe
    {
        wc = CStr::from_ptr(pointer).to_str()
        .expect("Encountered invalid char in a call to char_length")
        .chars().nth(0).unwrap();
    }
    wc.len_utf8()
}

/* Return the number of (multibyte) characters in the given string. */
#[no_mangle]
pub extern "C" fn mbstrlen(pointer: *const c_char) -> usize {
    let len: usize;
    unsafe
    {
        len = CStr::from_ptr(pointer).to_str()
        .expect("Encountered invalid char in a call to mbstrlen")
        .len();
    }
    len
}

/* Parse a multibyte character from buf.  Return the number of bytes
 * used.  If chr isn't NULL, store the multibyte character in it.  If
 * col isn't NULL, add the character's width (in columns) to it. */
// int parse_mbchar(const char *buf, char *chr, size_t *col)
// {
// 	int length;

// #ifdef ENABLE_UTF8
// 	/* If this is a UTF-8 starter byte, get the number of bytes of the character. */
// 	if ((signed char)*buf < 0) {
// 		length = mblen(buf, MAXCHARLEN);

// 		/* When the multibyte sequence is invalid, only take the first byte. */
// 		if (length <= 0)
// 			length = 1;
// 	} else
// #endif
// 		length = 1;

// 	/* When requested, store the multibyte character in chr. */
// 	if (chr != NULL)
// 		for (int i = 0; i < length; i++)
// 			chr[i] = buf[i];

// 	/* When requested, add the width of the character to col. */
// 	if (col != NULL) {
// 		/* If we have a tab, compute its width in columns based on the
// 		 * current value of col. */
// 		if (*buf == '\t')
// 			*col += tabsize - *col % tabsize;
// 		/* If we have a control character, it's two columns wide: one
// 		 * column for the "^", and one for the visible character. */
// 		else if (is_cntrl_mbchar(buf))
// 			*col += 2;
// 		/* If we have a normal character, get its width normally. */
// 		else if (length == 1)
// 			*col += 1;
// #ifdef ENABLE_UTF8
// 		else
// 			*col += mbwidth(buf);
// #endif
// 	}

// 	return length;
// }

/* Return the index in buf of the beginning of the multibyte character
 * before the one at pos. */
#[no_mangle]
pub extern "C" fn step_left(buf: *const c_char, pos: usize) -> usize
{
	if using_utf8() && cfg!(ENABLE_UTF8) {
        let mut before:usize;
        let mut charlen:usize = 0;

		if pos < 4{
            before = 0;
        }
		else {
			let ptr:*const c_char = unsafe{buf.offset(pos as isize)};

            /* Probe for a valid starter byte in the preceding four bytes. */
            unsafe {
                if *ptr.offset(1) > -65
                {
                    before = pos - 1;
                }
                else if *ptr.offset(2) > -65{
                    before = pos - 2;
                }
                else if *ptr.offset(3) > -65{
                    before = pos - 3;
                }
                else if *ptr.offset(4) > -65{
                    before = pos - 4;
                }
                else{
                    before = pos - 1;
                }
            }
		}

		/* Move forward again until we reach the original character,
		 * so we know the length of its preceding character. */
		while before < pos {
			charlen = char_length(unsafe{buf.offset(before as isize)});
			before += charlen;
		}

		before - charlen
    }
    else
    {
        if pos == 0 {0} else {pos - 1}
    }
}

/* Return the index in buf of the beginning of the multibyte character
 * after the one at pos. */
#[no_mangle]
pub extern "C" fn step_right(buf: *const c_char, pos: usize) -> usize {
    pos + char_length(unsafe{buf.offset(pos as isize)})
}

/* This function is equivalent to strcasecmp() for multibyte strings. */
#[no_mangle]
#[cfg(ENABLE_UTF8)]
pub extern "C" fn mbstrcasecmp(s1:*const c_char, s2:*const c_char) -> isize
{
	mbstrncasecmp(s1, s2, HIGHEST_POSITIVE)
}

/* This function is equivalent to strncasecmp() for multibyte strings. */
#[cfg(ENABLE_UTF8)]
#[no_mangle]
pub extern "C" fn mbstrncasecmp(mut s1:*const c_char,mut s2: *const c_char, mut n:usize) -> isize
{
    let mut wc1:char = '\0';//bad init
    let mut wc2:char = '\0';//bad init
    unsafe{

	    while *s1 != '\0' as i8 && *s2 != '\0' as i8 && n > 0 {
			let bad1:bool = match mbtowc(s1) {
                Ok(v)=>{wc1=v;true},
                Err(e)=>false
            };
			let bad2:bool = match mbtowc(s2) {
                Ok(v)=>{wc2=v;true},
                Err(e)=>false
            };

			if bad1 || bad2 {
				if *s1 != *s2 {
                    return (*s1 as u8 - *s2 as u8) as isize;
                }

				if bad1 != bad2 {
                    return if bad1 {1} else {-1};
                }
			} else {
				let difference = (wc1.to_lowercase().nth(0).unwrap() as u64 - wc2.to_lowercase().nth(0).unwrap() as u64) as isize;

				if difference != 0{
                    return difference;
                }
			}
            s1 = (s1 as usize + char_length(s1)) as *const c_char;
            s2 = (s2 as usize + char_length(s2)) as *const c_char;
			n-=1;
		}

        if n > 0 {(*s1 as u8 - *s2 as u8) as isize} else {0}
    }
}

/* This function is equivalent to strcasestr() for multibyte strings. */
#[cfg(ENABLE_UTF8)]
#[no_mangle]
pub extern "C" fn mbstrcasestr(mut haystack:*const c_char, needle:*const c_char) -> *const c_char
{
	unsafe {
		let needle_len = mbstrlen(needle);

		while *haystack != '\0' as i8 {
            if mbstrncasecmp(haystack, needle, needle_len) == 0
            {
                return haystack;
            }
            haystack = (haystack as usize + char_length(haystack)) as *const c_char;
		}

		ptr::null()
	} 
}

//Compare n bytes of the pointer
unsafe fn strncmp(a:*const c_char, b:*const c_char, n: usize) -> i8{
    for i in 1..(n+1) {
        if *a.offset(i as isize) != *b.offset(i as isize) {
            return *a.offset(i as isize) - *b.offset(i as isize)
        }
    }
    0
}

unsafe fn strncasecmp(a:*const c_char, b:*const c_char, n: usize) -> i8{
    for i in 1..(n+1) {
        if (*a.offset(i as isize) as u8 as char).to_lowercase().nth(0).unwrap() != (*b.offset(i as isize) as u8 as char).to_lowercase().nth(0).unwrap() {
            return (*a.offset(i as isize) as u8 as char).to_lowercase().nth(0).unwrap() as i8 - (*b.offset(i as isize) as u8 as char).to_lowercase().nth(0).unwrap() as i8;
        }
    }
    0
}

/* This function is equivalent to strstr(), except in that it scans the
 * string in reverse, starting at pointer. */
 #[no_mangle]
pub extern "C" fn revstrstr(haystack:*const c_char, needle:*const c_char,
		mut pointer:*const c_char) -> *const c_char
{
	let needle_len:usize = unsafe{CStr::from_ptr(needle)}.to_bytes().len();
	let tail_len:usize = unsafe{CStr::from_ptr(pointer)}.to_bytes().len();

	if tail_len < needle_len{
        pointer = (pointer as usize + tail_len - needle_len) as *const c_char;
    }

	while pointer >= haystack {
		if unsafe {strncmp(pointer, needle, needle_len)} == 0 {
            return pointer;
        }
        pointer= (pointer as usize - 1) as *const c_char;
	}

	ptr::null()
}

/* This function is equivalent to strcasestr(), except in that it scans
 * the string in reverse, starting at pointer. */
 #[no_mangle]
pub extern "C" fn revstrcasestr(haystack:*const c_char, needle:*const c_char,
		mut pointer:*const c_char) -> *const c_char
{
	let needle_len:usize = unsafe{CStr::from_ptr(needle)}.to_bytes().len();
	let tail_len:usize = unsafe{CStr::from_ptr(pointer)}.to_bytes().len();

	if tail_len < needle_len
    {
        pointer = (pointer as usize + tail_len - needle_len) as *const c_char;
    }

	while pointer as usize >= haystack as usize {
		if unsafe{strncasecmp(pointer, needle, needle_len)} == 0
		{
            return pointer;
        }
		pointer= (pointer as usize - 1) as *const c_char;
	}

	ptr::null()
}

// /* This function is equivalent to strcasestr() for multibyte strings,
//  * except in that it scans the string in reverse, starting at pointer. */
#[no_mangle]
pub extern "C" fn mbrevstrcasestr(haystack:*const c_char, needle:*const c_char,
		mut pointer:*const c_char) -> *const c_char
{
    unsafe {
		let needle_len = mbstrlen(needle);
		let tail_len = mbstrlen(pointer);

		if tail_len < needle_len
		{
            pointer = (pointer as usize + tail_len - needle_len) as *const c_char;
        }

		if pointer < haystack
		{
            return ptr::null();
        }

		loop {
			if mbstrncasecmp(pointer, needle, needle_len) == 0
			{
                return pointer;
            }

			if pointer == haystack
			{
                return ptr::null();
            }

			pointer = (haystack as usize + step_left(haystack, pointer as usize - haystack as usize)) as *const c_char;
		}
	} 
}

// #[cfg(any(not(NANO_TINY,ENABLE_JUSTIFY)]
// /* This function is equivalent to strchr() for multibyte strings. */
// pub extern "C" mbstrchr(string:*const c_char, chr:*const c_char) -> *const c_char
// {
// 	unsafe {
//         let mut bad_s = false;
//         let mut bad_c = false;
// 		char ws, wc;

// 		match mbtowc(chr) {
//             Ok(v) => wc = v,
// 			Err(e) => wc = *chr as u8;bad_c = true,
// 		}

// 		while *string != '\0' {
// 			let symlen = mbtowc(&ws, string, MAXCHARLEN);

// 			if (symlen < 0) {
// 				ws = (unsigned char)*string;
// 				bad_s = TRUE;
// 			}

// 			if (ws == wc && bad_s == bad_c)
// 				break;

// 			string += symlen;
// 		}

// 		if (*string == '\0')
// 			return NULL;

// 		return string;
// 	}
// }

// #ifndef NANO_TINY
/* Locate, in the given string, the first occurrence of any of
 * the characters in accept, searching forward. */
// char *mbstrpbrk(const char *string, const char *accept)
// {
// 	while (*string != '\0') {
// 		if (mbstrchr(accept, string) != NULL)
// 			return (char *)string;

// 		string += char_length(string);
// 	}

// 	return NULL;
// }

// /* Locate, in the string that starts at head, the first occurrence of any of
//  * the characters in accept, starting from pointer and searching backwards. */
// char *mbrevstrpbrk(const char *head, const char *accept, const char *pointer)
// {
// 	if (*pointer == '\0') {
// 		if (pointer == head)
// 			return NULL;
// 		pointer = head + step_left(head, pointer - head);
// 	}

// 	while (TRUE) {
// 		if (mbstrchr(accept, pointer) != NULL)
// 			return (char *)pointer;

// 		/* If we've reached the head of the string, we found nothing. */
// 		if (pointer == head)
// 			return NULL;

// 		pointer = head + step_left(head, pointer - head);
// 	}
// }
// #endif /* !NANO_TINY */

// #if defined(ENABLE_NANORC) && (!defined(NANO_TINY) || defined(ENABLE_JUSTIFY))
// /* Return TRUE if the given string contains at least one blank character,
//  * and FALSE otherwise. */
// bool has_blank_char(const char *string)
// {
// 	char symbol[MAXCHARLEN];

// 	while (*string != '\0') {
// 		string += parse_mbchar(string, symbol, NULL);

// 		if (is_blank_mbchar(symbol))
// 			return TRUE;
// 	}

// 	return FALSE;
// }
// #endif /* ENABLE_NANORC && (!NANO_TINY || ENABLE_JUSTIFY) */
/* #[cfg(all(ENABLE_NANORC,any(not(NANO_TINY),ENABLE_JUSTIFY)))]
pub extern "C" has_blank_char(string: *const c_char) -> bool {
    
} */

/* Return TRUE when the given string is empty or consists of only blanks. */
#[no_mangle]
pub extern "C" fn white_string(mut pointer: *const c_char) -> bool {
    unsafe{    
        while *pointer != ('\0' as c_char) && 
        (is_blank_mbchar(pointer) || *pointer == ('\r' as c_char)){
            pointer = (pointer as usize + char_length(pointer)) as *const c_char;
        }
        *pointer != 0
    }
}

/* Return TRUE if wc is valid Unicode, and FALSE otherwise. */
#[cfg(ENABLE_UTF8)]
#[no_mangle]
pub extern "C" fn is_valid_unicode(wc: char) -> bool {
    ((0 <= wc as i32 && wc as i32 <= 0xD7FF) ||
				(0xE000 <= wc as i32 && wc as i32 <= 0xFDCF) ||
				(0xFDF0 <= wc as i32 && wc as i32 <= 0xFFFD) ||
				(0xFFFF < wc as i32 && wc as i32 <= 0x10FFFF && (wc as i32 & 0xFFFF) <= 0xFFFD))
}

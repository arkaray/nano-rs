#![crate_type = "staticlib"]

// **TEMPORARY PLACE FOR CONSTANTS FROM NANO.H**
const DEL_CODE:i8 = 0x7f;

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
// Deprecate in rust
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
// bool is_alpha_mbchar(const char *c)
// {
// #ifdef ENABLE_UTF8
// 	if (use_utf8) {
// 		wchar_t wc;

// 		if (mbtowc(&wc, c, MAXCHARLEN) < 0)
// 			return FALSE;

// 		return iswalpha(wc);
// 	} else
// #endif
// 		return isalpha((unsigned char)*c);
// }

/* This function is equivalent to isalnum() for multibyte characters. */
// bool is_alnum_mbchar(const char *c)
// {
// #ifdef ENABLE_UTF8
// 	if (use_utf8) {
// 		wchar_t wc;

// 		if (mbtowc(&wc, c, MAXCHARLEN) < 0)
// 			return FALSE;

// 		return iswalnum(wc);
// 	} else
// #endif
// 		return isalnum((unsigned char)*c);
// }

/* This function is equivalent to isblank() for multibyte characters. */
// bool is_blank_mbchar(const char *c)
// {
// #ifdef ENABLE_UTF8
// 	if (use_utf8) {
// 		wchar_t wc;

// 		if (mbtowc(&wc, c, MAXCHARLEN) < 0)
// 			return FALSE;

// 		return iswblank(wc);
// 	} else
// #endif
// 		return isblank((unsigned char)*c);
// }

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
// bool is_cntrl_mbchar(const char *c)
// {
// #ifdef ENABLE_UTF8
// 	if (use_utf8) {
// 		return ((c[0] & 0xE0) == 0 || c[0] == 127 ||
// 				((signed char)c[0] == -62 && (signed char)c[1] < -96));
// 	} else
// #endif
// 		return (((unsigned char)*c & 0x60) == 0 || (unsigned char)*c == 127);
// }

/* This function is equivalent to ispunct() for multibyte characters. */
// bool is_punct_mbchar(const char *c)
// {
// #ifdef ENABLE_UTF8
// 	if (use_utf8) {
// 		wchar_t wc;

// 		if (mbtowc(&wc, c, MAXCHARLEN) < 0)
// 			return FALSE;

// 		return iswpunct(wc);
// 	} else
// #endif
// 		return ispunct((unsigned char)*c);
// }

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
// char control_rep(const signed char c)
// {
// 	if (c == DEL_CODE)
// 		return '?';
// 	else if (c == -97)
// 		return '=';
// 	else if (c < 0)
// 		return c + 224;
// 	else
// 		return c + 64;
// }
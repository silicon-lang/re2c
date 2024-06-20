/* Generated by re2c */
// re2rust $INPUT -o $OUTPUT

// Store u32 number in u64 during parsing to simplify overflow hadling.
struct State<'a> {
    str: &'a [u8],
    yycursor: usize,
    yymarker: usize,
    num: u64,
}



const ERROR: u64 = std::u32::MAX as u64 + 1; // overflow

macro_rules! maybe { // Convert the number from u64 to optional u32.
    ($n:expr) => { if $n < ERROR { Some($n as u32) } else { None } }
}

// Add digit with the given base, checking for overflow.
fn add(st: &mut State, offs: u8, base: u64) {
    let digit = unsafe { st.str.get_unchecked(st.yycursor - 1) } - offs;
    st.num = std::cmp::min(st.num * base + digit as u64, ERROR);
}

fn parse_u32(s: & [u8]) -> Option<u32> {
    assert_eq!(s.last(), Some(&0)); // expect null-terminated input

    let mut st = State {str: s, yycursor: 0, yymarker: 0, num: 0};

{
	#[allow(unused_assignments)]
	let mut yych : u8 = 0;
	let mut yystate : usize = 0;
	'yyl: loop {
		match yystate {
			0 => {
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				st.yycursor += 1;
				match yych {
					0x30 => {
						yystate = 2;
						continue 'yyl;
					}
					0x31 ..= 0x39 => {
						yystate = 4;
						continue 'yyl;
					}
					_ => {
						yystate = 1;
						continue 'yyl;
					}
				}
			}
			1 => { return None; },
			2 => {
				st.yymarker = st.yycursor;
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				match yych {
					0x42 |
					0x62 => {
						st.yycursor += 1;
						yystate = 5;
						continue 'yyl;
					}
					0x58 |
					0x78 => {
						st.yycursor += 1;
						yystate = 7;
						continue 'yyl;
					}
					_ => {
						yystate = 3;
						continue 'yyl;
					}
				}
			}
			3 => { return parse_oct(&mut st); },
			4 => {
				st.yycursor -= 1;
				{ return parse_dec(&mut st); }
			}
			5 => {
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				match yych {
					0x30 ..= 0x31 => {
						st.yycursor += 1;
						yystate = 8;
						continue 'yyl;
					}
					_ => {
						yystate = 6;
						continue 'yyl;
					}
				}
			}
			6 => {
				st.yycursor = st.yymarker;
				yystate = 3;
				continue 'yyl;
			}
			7 => {
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				match yych {
					0x30 ..= 0x39 |
					0x41 ..= 0x46 |
					0x61 ..= 0x66 => {
						st.yycursor += 1;
						yystate = 9;
						continue 'yyl;
					}
					_ => {
						yystate = 6;
						continue 'yyl;
					}
				}
			}
			8 => {
				st.yycursor -= 1;
				{ return parse_bin(&mut st); }
			}
			9 => {
				st.yycursor -= 1;
				{ return parse_hex(&mut st); }
			}
			_ => panic!("internal lexer error"),
		}
	}
}

}

fn parse_bin(st: &mut State) -> Option<u32> {
    'bin: loop {
{
	#[allow(unused_assignments)]
	let mut yych : u8 = 0;
	let mut yystate : usize = 0;
	'yyl: loop {
		match yystate {
			0 => {
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				st.yycursor += 1;
				match yych {
					0x30 ..= 0x31 => {
						yystate = 2;
						continue 'yyl;
					}
					_ => {
						yystate = 1;
						continue 'yyl;
					}
				}
			}
			1 => { return maybe!(st.num); },
			2 => { add(st, 48, 2); continue 'bin; },
			_ => panic!("internal lexer error"),
		}
	}
}
}
}

fn parse_oct(st: &mut State) -> Option<u32> {
    'oct: loop {
{
	#[allow(unused_assignments)]
	let mut yych : u8 = 0;
	let mut yystate : usize = 0;
	'yyl: loop {
		match yystate {
			0 => {
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				st.yycursor += 1;
				match yych {
					0x30 ..= 0x37 => {
						yystate = 2;
						continue 'yyl;
					}
					_ => {
						yystate = 1;
						continue 'yyl;
					}
				}
			}
			1 => { return maybe!(st.num); },
			2 => { add(st, 48, 8); continue 'oct; },
			_ => panic!("internal lexer error"),
		}
	}
}
}
}

fn parse_dec(st: &mut State) -> Option<u32> {
    'dec: loop {
{
	#[allow(unused_assignments)]
	let mut yych : u8 = 0;
	let mut yystate : usize = 0;
	'yyl: loop {
		match yystate {
			0 => {
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				st.yycursor += 1;
				match yych {
					0x30 ..= 0x39 => {
						yystate = 2;
						continue 'yyl;
					}
					_ => {
						yystate = 1;
						continue 'yyl;
					}
				}
			}
			1 => { return maybe!(st.num); },
			2 => { add(st, 48, 10); continue 'dec; },
			_ => panic!("internal lexer error"),
		}
	}
}
}
}

fn parse_hex(st: &mut State) -> Option<u32> {
    'hex: loop {
{
	#[allow(unused_assignments)]
	let mut yych : u8 = 0;
	let mut yystate : usize = 0;
	'yyl: loop {
		match yystate {
			0 => {
				yych = unsafe {*st.str.get_unchecked(st.yycursor)};
				st.yycursor += 1;
				match yych {
					0x30 ..= 0x39 => {
						yystate = 2;
						continue 'yyl;
					}
					0x41 ..= 0x46 => {
						yystate = 3;
						continue 'yyl;
					}
					0x61 ..= 0x66 => {
						yystate = 4;
						continue 'yyl;
					}
					_ => {
						yystate = 1;
						continue 'yyl;
					}
				}
			}
			1 => { return maybe!(st.num); },
			2 => { add(st, 48, 16); continue 'hex; },
			3 => { add(st, 55, 16); continue 'hex; },
			4 => { add(st, 87, 16); continue 'hex; },
			_ => panic!("internal lexer error"),
		}
	}
}
}
}

fn main() {
    assert_eq!(parse_u32(b"\0"), None);
    assert_eq!(parse_u32(b"1234567890\0"), Some(1234567890));
    assert_eq!(parse_u32(b"0b1101\0"), Some(13));
    assert_eq!(parse_u32(b"0x7Fe\0"), Some(2046));
    assert_eq!(parse_u32(b"0644\0"), Some(420));
    assert_eq!(parse_u32(b"9999999999\0"), None);
}

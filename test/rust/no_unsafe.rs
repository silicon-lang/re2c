/* Generated by re2c */
// re2rust $INPUT -o $OUTPUT --no-unsafe

// expect safe YYPEEK

{
	#[allow(unused_assignments)]
	let mut yych : YYCTYPE = 0;
	let mut yystate : usize = 0;
	loop {
		match yystate {
			0 => {
				if YYLESSTHAN {
					YYFILL
				}
				yych = YYPEEK;
				YYSKIP
				yystate = 1;
				continue;
			}
			1 => {
				yystate = 2;
				continue;
			}
			2 => {}
			_ => {
				panic!("internal lexer error")
			}
		}
	}
}


// expect `unsafe { ... }` wrapper around YYPEEK

{
	#[allow(unused_assignments)]
	let mut yych : YYCTYPE = 0;
	let mut yystate : usize = 0;
	loop {
		match yystate {
			0 => {
				if YYLESSTHAN {
					YYFILL
				}
				yych = unsafe {YYPEEK};
				YYSKIP
				yystate = 1;
				continue;
			}
			1 => {
				yystate = 2;
				continue;
			}
			2 => {}
			_ => {
				panic!("internal lexer error")
			}
		}
	}
}


// expect safe YYPEEK

{
	#[allow(unused_assignments)]
	let mut yych : YYCTYPE = 0;
	let mut yystate : usize = 0;
	loop {
		match yystate {
			0 => {
				if YYLESSTHAN {
					YYFILL
				}
				yych = YYPEEK;
				YYSKIP
				yystate = 1;
				continue;
			}
			1 => {
				yystate = 2;
				continue;
			}
			2 => {}
			_ => {
				panic!("internal lexer error")
			}
		}
	}
}


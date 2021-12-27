/* Generated by re2c */
// re2rust $INPUT -o $OUTPUT
// Test that re2c is able to lex Rust code block with braces and quotes.

'outer: loop { 
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
			2 => {
        let c0 = '\'';
        let c1 = '\x7F';
        let c2 = '}';
        let s = "}}";

        'inner: loop {
            let c3 = '\t';
            let c4 = '\u{7FFFF}';
            if c3 == c1 {
                break 'outer;
            }
            continue 'inner;
        }

        let c5 = 'ы';
        let c6 = b'a';
        continue 'outer;
    }
			_ => {
				panic!("internal lexer error")
			}
		}
	}
}
 }

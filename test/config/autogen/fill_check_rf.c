/* Generated by re2c */
// re2c $INPUT -o $OUTPUT -rfi



	switch (YYGETSTATE()) {
		case 0: goto yyFillLabel0;
		default: goto yy0;
	}
yy0:
	if (YYLIMIT <= YYCURSOR) {
		YYSETSTATE(0);
		YYFILL(1);
	}
yyFillLabel0:
	yych = *YYCURSOR;
	switch (yych) {
		case 'a': goto yy3;
		default: goto yy2;
	}
yy2:
	++YYCURSOR;
	YYSETSTATE(-1);
	{ x }
yy3:
	++YYCURSOR;
	YYSETSTATE(-1);
	{ a }


// re2c:yyfill:check = 0;

	switch (YYGETSTATE()) {
		case 1: goto yyFillLabel1;
		default: goto yy4;
	}
yy4:
	YYSETSTATE(1);
	YYFILL(1);
yyFillLabel1:
	yych = *YYCURSOR;
	switch (yych) {
		case 'a': goto yy7;
		default: goto yy6;
	}
yy6:
	++YYCURSOR;
	YYSETSTATE(-1);
	{ x }
yy7:
	++YYCURSOR;
	YYSETSTATE(-1);
	{ a }


// re2c:yyfill:check = 1;

	switch (YYGETSTATE()) {
		case 2: goto yyFillLabel2;
		default: goto yy8;
	}
yy8:
	if (YYLIMIT <= YYCURSOR) {
		YYSETSTATE(2);
		YYFILL(1);
	}
yyFillLabel2:
	yych = *YYCURSOR;
	switch (yych) {
		case 'a': goto yy11;
		default: goto yy10;
	}
yy10:
	++YYCURSOR;
	YYSETSTATE(-1);
	{ x }
yy11:
	++YYCURSOR;
	YYSETSTATE(-1);
	{ a }



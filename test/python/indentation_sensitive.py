# Generated by re2c
# re2py $INPUT -o $OUTPUT


    yystate = 0
    while True:
        match yystate:
            case 0:
                if YYLESSTHAN:
                    YYFILL
                yych = YYPEEK
                YYSKIP
                match yych:
                    case 0x61:
                        yystate = 2
                        continue
                    case 0x62:
                        yystate = 3
                        continue
                    case 0x63:
                        yystate = 4
                        continue
                    case 0x64:
                        yystate = 5
                        continue
                    case 0x65:
                        yystate = 6
                        continue
                    case 0x66:
                        yystate = 7
                        continue
                    case 0x67:
                        yystate = 8
                        continue
                    case 0x68:
                        yystate = 9
                        continue
                    case 0x69:
                        yystate = 10
                        continue
                    case 0x6A:
                        yystate = 11
                        continue
                    case 0x6B:
                        yystate = 12
                        continue
                    case 0x6C:
                        yystate = 13
                        continue
                    case 0x6D:
                        yystate = 14
                        continue
                    case 0x6E:
                        yystate = 15
                        continue
                    case 0x6F:
                        yystate = 16
                        continue
                    case 0x70:
                        yystate = 17
                        continue
                    case _:
                        yystate = 1
                        continue
            case 1:
                ***
            case 2:
                aaa
            case 3:
                bbb 1
                  bbb 2
                bbb 3
            case 4:
                ccc 1
                ccc 2
                  ccc 3
            case 5:
                ddd 1
                  ddd 2
                    ddd 3
            case 6:
                eee
            case 7:
                fff 1
                  fff 2
                fff 3
            case 8:
                ggg 1
                  ggg 2
                    ggg 3
            case 9:
            case 10:
            case 11:
            case 12:
            case 13:
            case 14:
            case 15:
            case 16:
                ooo
            case 17:
                ppp
            case _:
                raise "internal lexer error"


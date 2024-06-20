# Generated by re2c
# re2py $INPUT -o $OUTPUT

from collections import namedtuple
from enum import Enum
import os

BUFSIZE = 4096

SemVer = namedtuple('SemVer', 'major minor patch')

class State:
    def __init__(self, fname):
        self.file = open(fname, "rb")
        self.str = bytearray(BUFSIZE)
        self.yylimit = BUFSIZE - 1 # exclude terminating null
        self.yycursor = self.yylimit
        self.yymarker = self.yylimit
        self.token = self.yylimit
        self.eof = False
        
        self.yyt1 = -1
        self.yyt2 = -1
        self.yyt3 = -1

    def __del__(self):
        self.file.close()

class Status(Enum):
    OK = 0
    EOF = 1
    LONG_LEXEME = 2

def fill(st):
    if st.eof:
        return Status.EOF

    # Error: lexeme too long. In real life could reallocate a larger buffer.
    if st.token < 1:
        return Status.LONG_LEXEME

    # Shift buffer contents (discard everything up to the current token).
    st.str = st.str[st.token:st.yylimit]
    st.yycursor -= st.token;
    st.yymarker -= st.token;
    st.yylimit -= st.token;
    st.token = 0;

    # Fill free space at the end of buffer with new data from file.
    bytes = st.file.read(BUFSIZE - st.yylimit - 1) # -1 for sentinel
    if not bytes:
        st.eof = True # end of file
    else:
        st.yylimit += len(bytes);
        st.str += bytes

    st.str += b'\0' # append sentinel

    return Status.OK

def lex(st, count):
    vers = []
    while True:
        st.token = st.yycursor
        
        yystate = 0
        while True:
            match yystate:
                case 0:
                    yych = st.str[st.yycursor]
                    if yych <= 0x00:
                        if st.yylimit <= st.yycursor:
                            if fill(st) == Status.OK:
                                yystate = 0
                                continue
                            yystate = 11
                            continue
                        st.yycursor += 1
                        yystate = 1
                        continue
                    if yych <= 0x2F:
                        st.yycursor += 1
                        yystate = 1
                        continue
                    if yych <= 0x39:
                        st.yycursor += 1
                        yystate = 3
                        continue
                    st.yycursor += 1
                    yystate = 1
                    continue
                case 1:
                    yystate = 2
                    continue
                case 2:
                    return None
                case 3:
                    st.yymarker = st.yycursor
                    yych = st.str[st.yycursor]
                    if yych <= 0x2E:
                        if yych <= 0x00:
                            if st.yylimit <= st.yycursor:
                                if fill(st) == Status.OK:
                                    yystate = 3
                                    continue
                            yystate = 2
                            continue
                        if yych <= 0x2D:
                            yystate = 2
                            continue
                        st.yycursor += 1
                        yystate = 4
                        continue
                    else:
                        if yych <= 0x2F:
                            yystate = 2
                            continue
                        if yych <= 0x39:
                            st.yycursor += 1
                            yystate = 6
                            continue
                        yystate = 2
                        continue
                case 4:
                    yych = st.str[st.yycursor]
                    if yych <= 0x00:
                        if st.yylimit <= st.yycursor:
                            if fill(st) == Status.OK:
                                yystate = 4
                                continue
                        yystate = 5
                        continue
                    if yych <= 0x2F:
                        yystate = 5
                        continue
                    if yych <= 0x39:
                        yyt1 = st.yycursor
                        st.yycursor += 1
                        yystate = 7
                        continue
                    yystate = 5
                    continue
                case 5:
                    st.yycursor = st.yymarker
                    yystate = 2
                    continue
                case 6:
                    yych = st.str[st.yycursor]
                    if yych <= 0x2E:
                        if yych <= 0x00:
                            if st.yylimit <= st.yycursor:
                                if fill(st) == Status.OK:
                                    yystate = 6
                                    continue
                            yystate = 5
                            continue
                        if yych <= 0x2D:
                            yystate = 5
                            continue
                        st.yycursor += 1
                        yystate = 4
                        continue
                    else:
                        if yych <= 0x2F:
                            yystate = 5
                            continue
                        if yych <= 0x39:
                            st.yycursor += 1
                            yystate = 6
                            continue
                        yystate = 5
                        continue
                case 7:
                    yych = st.str[st.yycursor]
                    if yych <= 0x2D:
                        if yych <= 0x00:
                            if st.yylimit <= st.yycursor:
                                if fill(st) == Status.OK:
                                    yystate = 7
                                    continue
                            yystate = 5
                            continue
                        if yych != 0x0A:
                            yystate = 5
                            continue
                        yyt2 = st.yycursor
                        yyt3 = -1
                        st.yycursor += 1
                        yystate = 8
                        continue
                    else:
                        if yych <= 0x2E:
                            yyt2 = st.yycursor
                            st.yycursor += 1
                            yystate = 9
                            continue
                        if yych <= 0x2F:
                            yystate = 5
                            continue
                        if yych <= 0x39:
                            st.yycursor += 1
                            yystate = 7
                            continue
                        yystate = 5
                        continue
                case 8:
                    t2 = yyt1
                    t3 = yyt2
                    t4 = yyt3
                    t1 = yyt1
                    t1 -= 1
                    major = int(st.str[st.token:t1]);
                    minor = int(st.str[t2:t3]);
                    patch = int(st.str[t4:st.yycursor - 1]) if t4 != -1 else 0
                    vers.append(SemVer(major, minor, patch))
                    break
                case 9:
                    yych = st.str[st.yycursor]
                    if yych <= 0x00:
                        if st.yylimit <= st.yycursor:
                            if fill(st) == Status.OK:
                                yystate = 9
                                continue
                        yystate = 5
                        continue
                    if yych <= 0x2F:
                        yystate = 5
                        continue
                    if yych >= 0x3A:
                        yystate = 5
                        continue
                    yyt3 = st.yycursor
                    st.yycursor += 1
                    yystate = 10
                    continue
                case 10:
                    yych = st.str[st.yycursor]
                    if yych <= 0x0A:
                        if yych <= 0x00:
                            if st.yylimit <= st.yycursor:
                                if fill(st) == Status.OK:
                                    yystate = 10
                                    continue
                            yystate = 5
                            continue
                        if yych <= 0x09:
                            yystate = 5
                            continue
                        st.yycursor += 1
                        yystate = 8
                        continue
                    else:
                        if yych <= 0x2F:
                            yystate = 5
                            continue
                        if yych <= 0x39:
                            st.yycursor += 1
                            yystate = 10
                            continue
                        yystate = 5
                        continue
                case 11:
                    return vers
                case _:
                    raise "internal lexer error"


def main():
    fname = "input"
    verstr = b"1.22.333\n"
    expect = [SemVer(1, 22, 333)] * BUFSIZE

    # Prepare input file.
    f = open(fname, "wb")
    for i in range(BUFSIZE):
        f.write(verstr)
    f.close()

    # Run lexer on the prepared file.
    st = State(fname)
    assert lex(st, 0) == expect

    # Cleanup.
    os.remove(fname)

if __name__ == '__main__':
    main()

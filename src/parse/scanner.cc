#include <stdio.h>
#include <algorithm>
#include "src/util/c99_stdint.h"
#include <limits>
#include <string.h>

#include "src/msg/msg.h"
#include "src/options/opt.h"
#include "src/parse/scanner.h"
#include "src/debug/debug.h"
#include "src/util/file_utils.h"


namespace re2c {

const char *const Scanner::ENDPOS = (const char*) std::numeric_limits<uint64_t>::max();

Scanner::~Scanner()
{
    for (size_t i = files.size(); i --> 0; ) {
        delete files[i];
    }
}

size_t Scanner::get_input_index() const
{
    // Find index of the current input file: the one corresponding to
    // buffer fragment that contains cursor.
    size_t i = files.size();
    DASSERT(i > 0);
    for (;;) {
        --i;
        Input *in = files[i];
        if (i == 0 || (cur >= in->so && cur <= in->eo)) break;
    }
    return i;
}

bool Scanner::open(const std::string &filename, const std::string *parent)
{
    Input *in = new Input(msg.filenames.size());
    files.push_back(in);
    if (!in->open(filename, parent, globopts->incpaths)) {
        return false;
    }
    filedeps.insert(in->escaped_name);
    msg.filenames.push_back(in->escaped_name);
    return true;
}

bool Scanner::include(const std::string &filename, char *at)
{
    // This function is called twice for each included file: first time when opening the
    // file, and second time when it has been fully read and can be closed. Second time
    // is needed to generate a line directive marking the end of the included file and the
    // continuation of the parent file. To get the second call, we "unread" include
    // directive on the first call (essentially just don't move token pointer to cursor)
    // and let the lexer scan it twice. To differentiate the first and the second times,
    // we compare the topmost file on stack with the inlude file (after popping all
    // finished files, as there may be nested includes). This logic can't handle recursive
    // self-includes, but they would be erroneous anyway.
    pop_finished_files();
    if (files.back()->name == filename) return true;

    // get name of the current file (before unreading)
    const size_t fidx = get_input_index();
    DASSERT(fidx < files.size());
    const std::string &parent = files[fidx]->escaped_name;

    // Unread buffer tail: we'll return to it later. In the buffer nested files go before
    // outer files. In the file stack, however, outer files go before nested files (nested
    // are at the top). We want to break from the unreading cycle early, therefore we go
    // in reverse order of file offsets in buffer and break as soon as the end offset is
    // less than cursor (current position). `at` points at the start of include directive.
    for (size_t i = 0; i < files.size(); ++i) {
        Input *in = files[i];
        if (in->so >= at) {
            // unread whole fragment
            fseek(in->file, in->so - in->eo, SEEK_CUR);
            in->so = in->eo = ENDPOS;
        }
        else if (in->eo >= at) {
            // fragment on the boundary, unread partially
            fseek(in->file, at - in->eo, SEEK_CUR);
            in->eo = cur - 1;
        }
        else {
            // the rest has been consumed already
            break;
        }
    }

    // open new file and place place at the top of stack
    if (!open(filename, &parent)) {
        return false;
    }

    // refill buffer (discard everything up to cursor, clear EOF)
    lim = cur = mar = ctx = tok = ptr = pos = bot + BSIZE;
    eof = NULL;
    return fill(BSIZE);
}

bool Scanner::read(size_t want)
{
    DASSERT(!files.empty());
    for (size_t i = files.size(); i --> 0; ) {
        Input *in = files[i];
        const size_t have = fread(lim, 1, want, in->file);
        in->so = lim;
        lim += have;
        in->eo = lim;
        want -= have;

        // buffer filled
        if (want == 0) return true;
    }
    return false;
}

void Scanner::shift_ptrs_and_fpos(ptrdiff_t offs)
{
    // shift buffer pointers
    shift_ptrs(offs);

    // shift file pointers
    for (size_t i = files.size(); i --> 0; ) {
        Input *in = files[i];
        if (in->so == ENDPOS && in->eo == ENDPOS) break;
        DASSERT(in->so != ENDPOS && in->eo != ENDPOS);
        in->so += offs;
        in->eo += offs;
    }
}

void Scanner::pop_finished_files()
{
    // Pop all files that have been fully processed (file upper bound
    // in buffer points before the first character of current lexeme),
    // except for the first (main) file which must always remain at the
    // bottom of the stack.
    size_t i = files.size();
    DASSERT(i > 0);
    for (;;) {
        --i;
        Input *in = files[i];
        if (i == 0 || in->eo >= tok) break;
        files.pop_back();
        delete in;
    }
}

bool Scanner::fill(size_t need)
{
    if (eof) return false;

    pop_finished_files();

    DASSERT(bot <= tok && tok <= lim);
    size_t free = static_cast<size_t>(tok - bot);
    size_t copy = static_cast<size_t>(lim - tok);

    if (free >= need) {
        memmove(bot, tok, copy);
        shift_ptrs_and_fpos(-static_cast<ptrdiff_t>(free));
    }
    else {
        BSIZE += std::max(BSIZE, need);
        char * buf = new char[BSIZE + YYMAXFILL];
        if (!buf) {
            error("out of memory");
            exit(1);
        }

        memmove(buf, tok, copy);
        shift_ptrs_and_fpos(buf - tok);
        delete [] bot;
        bot = buf;

        free = BSIZE - copy;
    }

    DASSERT(lim + free <= bot + BSIZE);
    if (!read(free)) {
        eof = lim;
        memset(lim, 0, YYMAXFILL);
        lim += YYMAXFILL;
    }

    return true;
}

bool Scanner::gen_dep_file() const
{
    const std::string &fname = globopts->dep_file;
    if (fname.empty()) return true;

    FILE *file = fopen(fname.c_str(), "w");
    if (file == NULL) {
        error("cannot open dep file %s", fname.c_str());
        return false;
    }

    fprintf(file, "%s:", escape_backslashes(globopts->output_file).c_str());
    for (std::set<std::string>::const_iterator i = filedeps.begin();
        i != filedeps.end(); ++i) {
        fprintf(file, " %s", i->c_str());
    }
    fprintf(file, "\n");

    fclose(file);
    return true;
}

uint32_t Scanner::decode(const char *str) const
{
    return globopts->input_encoding == Enc::ASCII
        ? static_cast<uint8_t>(str[0])
        : utf8::decode_unsafe(str);
}

} // namespace re2c

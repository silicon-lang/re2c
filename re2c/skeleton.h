#ifndef _skeleton_h
#define _skeleton_h

#include <map>
#include <vector>

#include "basics.h"
#include "dfa.h"

namespace re2c
{

struct SkeletonState
{
	typedef std::map<SkeletonState *, std::vector<std::pair<uint, uint> > > go_t;
	go_t go;
	uint rule;
	bool visited;
};

struct Prefix
{
	std::vector<uint> chars;
	uint length;
	uint rule;
	Prefix (const std::vector<uint> & cs, uint l, uint r)
		: chars (cs)
		, length (l)
		, rule (r)
	{}
};

struct Result
{
	uint processed;
	uint consumed;
	uint rule;
	Result (uint p, uint c, uint r)
		: processed (p)
		, consumed (c)
		, rule (r)
	{}
};

struct Skeleton
{
	SkeletonState * states;
	Skeleton (const DFA & dfa);
	~Skeleton ();
	unsigned long count ();
	void emit_data (DataFile & o);
};

unsigned long count_data (SkeletonState * s, unsigned long count);
void generate_data (DataFile & o, uint ind, SkeletonState * s, const std::vector<Prefix> & xs, std::vector<Result> & ys);
void skeleton_emit_prolog (OutputFile & o, uint ind, const char * data_name);
void skeleton_emit_epilog (OutputFile & o, uint ind);

} // namespace re2c

#endif // _skeleton_h
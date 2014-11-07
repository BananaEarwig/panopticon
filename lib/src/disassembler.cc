#include <panopticon/disassembler.hh>

using namespace po;
using namespace std;

po::tokpat_error::tokpat_error(std::string w)
: std::invalid_argument(w)
{}

po::token_expr::token_expr(std::string const& s)
: _u(terminal{s}) {}

/*po::token_expr::token_expr(unsigned long long i)
: _u(terminal{i}) {}*/

po::token_expr::token_expr(token_expr const& e1,token_expr const& e2)
{
	_u = conjunction(e1,e2);
}

po::token_expr::token_expr(token_expr_union const& e)
: _u(e) {}

po::token_expr po::operator*(po::token_expr const& e)
{
	return token_expr::token_expr_union(token_expr::option(e));
}

po::token_expr po::operator"" _e(char const* s,unsigned long l)
{
	return token_expr(std::string(s,l));
}

po::token_expr po::operator"" _e(unsigned long long l)
{
	return token_expr(l);
}

po::token_expr po::operator>>(po::token_expr const& e1,po::token_expr const& e2)
{
	token_expr::conjunction t3(e1,e2);
	return token_expr(token_expr::token_expr_union(t3));
}

#include <string>
#include <algorithm>
#include <sstream>
#include <cctype>	// isalnum

#include <value.hh>

using namespace po;
using namespace std;

rvalue::rvalue(void) 
: d{0}
{
	d.simple.tag = UndefinedValueTag;
	d.simple.rest = 0;
}

rvalue::rvalue(const rvalue &r)
: d{0}
{
	if(r.is_memory())
		assign_memory(r.to_memory());
	else if(r.is_constant())
		assign_constant(r.to_constant());
	else
		d.all = r.d.all;
}

rvalue &rvalue::operator=(const rvalue &r)
{
	if(&r == this)
		return *this;

	if(is_memory())
		destruct_memory();
	else if(is_constant())
		destruct_constant();

	if(r.is_memory())
		assign_memory(r.to_memory());
	else if(r.is_constant())
		assign_constant(r.to_constant());
	else
		d.all = r.d.all;

	return *this;
}

void rvalue::assign_memory(const class po::memory &r)
{
	memory_priv *p = (memory_priv *)(r.d.simple.rest << 2);
	
	++p->usage;
	d.simple.rest = (uint64_t)(p) >> 2;
	d.simple.tag = MemoryValueTag;
}

void rvalue::assign_constant(const class po::constant &r)
{
	constant_priv *p = (constant_priv *)(r.d.simple.rest << 2);
	
	++p->usage;
	d.simple.rest = (uint64_t)(p) >> 2;
	d.simple.tag = ConstantValueTag;
}

rvalue::~rvalue(void) 
{
	if(is_memory())
		destruct_memory();
	else if(is_constant())
		destruct_constant();
}

void rvalue::destruct_memory(void)
{
	memory_priv *p = (memory_priv *)(d.simple.rest << 2);
	if(!--p->usage)
		delete p;
}

void rvalue::destruct_constant(void)
{
	constant_priv *p = (constant_priv *)(d.simple.rest << 2);
	if(!--p->usage)
		delete p;
}

ostream &po::operator<<(ostream &os, const rvalue &r)
{
	switch(r.tag())
	{
	case rvalue::UndefinedValueTag: os << string("⊥"); return os;
	case rvalue::ConstantValueTag: 	os << r.to_constant().content(); return os;
	case rvalue::VariableValueTag:
	{
		const variable &v = r.to_variable();

		// base name
		os << v.name();
		
		// subscript
		if(v.subscript() >= 0)
		{
			string t = to_string(v.subscript());

			for_each(t.cbegin(),t.cend(),[&os](const char c)
			{
				switch(c)
				{
					case '0': os << "₀"; break;
					case '1': os << "₁"; break;
					case '2': os << "₂"; break;
					case '3': os << "₃"; break;
					case '4': os << "₄"; break;
					case '5': os << "₅"; break;
					case '6': os << "₆"; break;
					case '7': os << "₇"; break;
					case '8': os << "₈"; break;
					case '9': os << "₉"; break;
					default: assert(false);
				}
			});
		}
		return os;
	}
	case rvalue::MemoryValueTag:
	{
		const memory &m = r.to_memory();
		
		// name and offset
		os << m.name() << "[" << m.offset() << ";" << m.bytes();

		// endianess
		switch(m.endianess())
		{
			case memory::LittleEndian: os << "←"; break;
			case memory::BigEndian: os << "→"; break;
			default: os << "?"; break;
		}

		os << "]";
		return os;
	}
	default:
		throw value_exception("Unknown value tag " + to_string(r.tag()));
	}
}

bool rvalue::operator<(const rvalue &b) const
{
	if(is_memory() && b.is_memory())
	{
		const po::memory &am = to_memory();
		const po::memory &bm = b.to_memory();

		if(am.name() != bm.name())
			return am.name() < bm.name();
		else if(am.offset() != bm.offset())
			return am.offset() < bm.offset();
		else if(am.endianess() != bm.endianess())
			return am.endianess() < bm.endianess();
		else
			return am.bytes() < bm.bytes();
	}
	else if(is_constant() && b.is_constant())
	{
		const po::constant &ac = to_constant();
		const po::constant &bc = b.to_constant();

		if(ac.content() != bc.content())
			return ac.content() < bc.content();
		else 
			return ac.width() < bc.width();
	}
	else
		return d.all < b.d.all;
}

bool rvalue::operator==(const rvalue &b) const
{	
	if(is_memory() && b.is_memory())
	{
		const po::memory &am = to_memory();
		const po::memory &bm = b.to_memory();

		return am.name() == bm.name() &&
					 am.offset() == bm.offset() &&
					 am.endianess() == bm.endianess() &&
					 am.bytes() == bm.bytes();
	}
	if(is_constant() && b.is_constant())
	{
		return to_constant().content() == b.to_constant().content();
	}
	else
		return d.all == b.d.all;
}

bool rvalue::operator!=(const rvalue &b) const
{
	return !(*this == b);
}
rvalue::Tag rvalue::tag(void) const { return (Tag)d.simple.tag; }
	
bool rvalue::is_constant(void) const { return d.simple.tag == ConstantValueTag; }
bool rvalue::is_undefined(void) const { return d.simple.tag == UndefinedValueTag; }
bool rvalue::is_variable(void) const { return d.simple.tag == VariableValueTag; }
bool rvalue::is_memory(void) const { return d.simple.tag == MemoryValueTag; }
bool rvalue::is_lvalue(void) const { return is_memory() || is_variable(); }

const constant &rvalue::to_constant(void) const 
{ 
	if(!is_constant())
		throw value_exception("Cast to constant from invalid type");
	return *reinterpret_cast<const class constant *>(this); 
}

const variable &rvalue::to_variable(void) const 
{
	if(!is_variable())
		throw value_exception("Cast to variable from invalid type");
	return *reinterpret_cast<const class variable *>(this); 
}

const memory &rvalue::to_memory(void) const 
{ 
	if(!is_memory())
		throw value_exception("Cast to memory from invalid type");
	return *reinterpret_cast<const class memory *>(this); 
}

constant::constant(uint64_t n, uint16_t w)
{
	constant_priv *p = new constant_priv();
	p->usage = 1;
	p->width = 0;
	p->content = n;
	p->width = w;

	if(w)
		p->content &= ((uint64_t)1u << w) - 1u;
	else
		p->content = 0;

	d.simple.rest = (uint64_t)(p) >> 2;
	d.simple.tag = ConstantValueTag;
}

uint64_t constant::content(void) const
{
	return ((constant_priv *)(d.simple.rest << 2))->content;
}

uint16_t constant::width(void) const
{
	return ((constant_priv *)(d.simple.rest << 2))->width;
}

uint64_t po::flsll(uint64_t x)
{
	uint64_t ret = 0;
	
	while(x)
	{
		x >>= 1;
		++ret;
	}
	return ret;
}

variable::variable(string b, uint16_t w, int s)
{
	if(b.size() >= 6)
		throw value_exception("Variable names are limited to five characters");
	if(!all_of(b.begin(),b.end(),[&](const char &c) { return !(c & 0x80); }))
		throw value_exception("Variable names are limited to ASCII characters");
	if(w >= 0x100)
		throw value_exception("Variable width is limited to 255 bits");
	
	d.name.tag = VariableValueTag;
	d.name.n1 = b.size() >= 1 ? b.data()[0] : 0;
	d.name.n2 = b.size() >= 2 ? b.data()[1] : 0;
	d.name.n3 = b.size() >= 3 ? b.data()[2] : 0;
	d.name.n4 = b.size() >= 4 ? b.data()[3] : 0;
	d.name.n5 = b.size() >= 5 ? b.data()[4] : 0;
	//d.name.n6 = b.size() >= 6 ? b.data()[5] : 0;
	d.name.sub = (s < 0 ? 0xffff : (unsigned int)s);
	d.name.width = w;
}

string variable::name(void) const
{
	stringstream ss;

	ss << (d.name.n1 ? string(1,(char)d.name.n1) : "") 
		 << (d.name.n2 ? string(1,(char)d.name.n2) : "") 
		 << (d.name.n3 ? string(1,(char)d.name.n3) : "") 
		 << (d.name.n4 ? string(1,(char)d.name.n4) : "") 
		 << (d.name.n5 ? string(1,(char)d.name.n5) : "");
		// << (d.name.n6 ? string(1,(char)d.name.n6) : "");
	return ss.str();
}

int variable::subscript(void) const
{
	return d.name.sub != 0xffff ? d.name.sub : -1;
}

uint16_t variable::width(void) const
{
	return d.name.width;
}

memory_priv::memory_priv(void)
: usage(0), offset(), bytes(0), endianess(memory::NoEndian), name("")
{}

memory::memory(rvalue o, uint16_t w, Endianess e, string n)
{
	if(n.empty())
		throw value_exception("Memory bank name must not be empty");
	if(!w)
		throw value_exception("Memory bytes read must be non-zero");

	memory_priv *p = new memory_priv();
	p->offset = o;
	p->bytes = w;
	p->endianess = e;
	p->name = n;
	p->usage = 1;

	d.simple.rest = (uint64_t)(p) >> 2;
	d.simple.tag = MemoryValueTag;
}

const rvalue &memory::offset(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->offset; 
}

uint16_t memory::bytes(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->bytes; 
}

memory::Endianess memory::endianess(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->endianess; 
}

const string &memory::name(void) const 
{ 
	return ((memory_priv *)(d.simple.rest << 2))->name; 
}

oturtlestream &po::operator<<(oturtlestream &os, rvalue r)
{
	switch(r.tag())
	{
		case rvalue::UndefinedValueTag: os << "[rdf:type po:Undefined]"; return os;
		case rvalue::ConstantValueTag: 	os << "[rdf:type po:Constant; po:value " << r.to_constant().content() << "]"; return os;
	case rvalue::VariableValueTag:
	{
		const variable &v = r.to_variable();
		os << "[rdf:type po:Variable; po:name \"" << v.name() << "\"; " 
			 << (v.subscript() >= 0 ? "po:subscript " + to_string(v.subscript()) + "; " : "") 
			 << "po:width " << v.width() << "]";
		return os;
	}
	case rvalue::MemoryValueTag:
	{
		const memory &m = r.to_memory();
		
		os << "[rdf:type po:Memory; " 
			 << "po:name \"" << m.name() << "\"^^xsd:string; "
			 << "po:offset " << m.offset() << "; "
			 << "po:bytes " << m.bytes() << "; "
			 << "po:endianess ";

		// endianess
		switch(m.endianess())
		{
			case memory::LittleEndian: os << "po:little-endian; "; break;
			case memory::BigEndian: os << "po:big-endian; "; break;
			default: assert(false);
		}

		os << "]";
		return os;
	}
	default:
		throw value_exception("Unknown value tag " + to_string(r.tag()));
	}
}

rvalue rvalue::unmarshal(const rdf::node &node, const rdf::storage &store)
{
	/*
	rdf::node undef = store.single("po:undefined"),
						const_type = store.single("po:Constant"),
						var_type = store.single("po:Variable"),
						mem_type = store.single("po:Memory");

	if(node == undef)
	{
		return undefined();
	}
	else
	{
		string str = node.to_string();
		rdf::node type = node.type();
		
		if(type == const_type)
		{
			auto sub_idx = str.rfind('_');

			if(sub_idx != string::npos)
				return variable(str.substr(0,sub_idx),1,stoll(str.substr(sub_idx + 1)));
			else
				return variable(str,1);
		}
		else if(type == const_type)
		{
			return constant(stoull(str));
		}
		else if(type == mem_type)
		{
			auto sub_idx = str.find('[');
			auto bytes_idx = str.find(';');
			auto endianess_idx = str.find(',');

			rvalue offr = rvalue::unmarshalstr.substr(sub_idx,bytes_idx - sub_idx);
			unsigned long bytes = stoul(str.substr(bytes_idx,endianess_idx - bytes_idx));
*/
	return undefined();
}

value_exception::value_exception(const string &w) : runtime_error(w) {}

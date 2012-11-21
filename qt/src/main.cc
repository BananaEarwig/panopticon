#include <QApplication>
#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

#include <deflate.hh>
#include <avr/avr.hh>
#include <flowgraph.hh>

#include <window.hh>

void decode(std::vector<uint16_t> &bytes)
{
	po::flow_ptr flow = po::avr::disassemble(bytes,0);
	//cout << graphviz(flow) << endl;
	std::cout << po::turtle(flow) << std::endl;
}

int main(int argc, char *argv[])
{
	if(argc <= 1)
	{
		QApplication app(argc,argv);
		Window win;

		win.show();
		app.exec();
		return 1;
	}

	int fn = 1;
	while(fn < argc)
	{
		std::ifstream f(argv[fn]);
		std::vector<uint16_t> bytes;

		if (f.bad())
        std::cout << "I/O error while reading" << std::endl;
    else if (f.fail())
        std::cout << "Non-integer data encountered" << std::endl;
		else 
		{
			while(f.good() && !f.eof())
			{
				uint16_t c;
				f.read((char *)&c,sizeof(c));
				bytes.push_back(c);
			}
			decode(bytes);
		}

		++fn;
	}

	return 0;
}

Intro
=====

Panopticon is a cross platform disassembler for reverse engineering.
It consists of a C++ library for disassembling, analysing decompiling
and patching binaries for various platforms and instruction sets.

Panopticon comes with GUI for browsing control flow graphs, displaying
analysis results, controlling debugger instances and editing the on-disk
as well as in-memory representation of the program.

Building
========

In order to compile Panopticon the following needs to be installed first:

- Qt 5.3
- CMake 2.8
- g++ 4.7 or Clang 3.4
- Boost 1.53
- Kyoto Cabinet 1.2.76
- libarchive 3.1.2
- googletest 1.6.0 (only needed for the test suite)

Linux
-----

First install the prerequisites using your package manager. For Ubuntu
13.10 and 14.04 it's ``apt-get install g++ cmake libboost-dev kyotocabinet-dev libarchive-dev gtest-dev libqt5-dev``
, for Fedora 20 it's ``yum install g++ cmake libboost-devel kyotocabinet-devel libarchive-devel gtest-devel libqt5-devel``.

After that clone the repository onto disk, create a build directory and
call cmake and the path to the source as argument. Compile the project
using GNU Make.

```bash
git clone https://github.com/das-labor/panopticon.git
mkdir panop-build
cd panop-build
cmake ../panopticon
make -j4
sudo make install
```

Windows
-------

After installing the prerequisites on Windows use the CMake GUI to
generate Visual Studio project files or Mingw Makefiles. Panopticon
can be compiled using VC++ 2013 or Mingw g++.

Running
=======

The current version only supports AVR and has no ELF or PE loader yet.
To test Panopticon you need relocated AVR code. Such a file is prepared in
``lib/test/sosse``.

```bash
qt/qtpanopticon -a ../panopticon/lib/test/sosse
```

Or, you can start Panopticon without command line parameters and
select the test file manually by starting a new session.

Contributing
============

Panopticon is licensed under GPLv3 and is Free Software. Hackers are
always welcome. See http://panopticon.re for our wiki and issue tracker.

Panopticon consists of two sub projects: libpanopticon and qtpanopticon.
The libpanopticon resides in the lib/ directory inside the repository. It
implements all disassembling and analysis functionality.
The libpanopticon has a test suite that can be found in lib/test/ after compilation.
The library is documented using Doxygen. To generate an API documentation in HTML install
Doxygen and call ``doxygen doc/doxyfile`` from inside the repository. The documentation is
written to ``doc/html/``.

The qtpanopticon application is a Qt5 GUI for libpanopticon. The front end uses
QtQuick2 that interacts with libpanopticon using a thin C++ interface (the
Session, Panopticon, LinearModel and ProcedureModel classes). For the graph view
qtpanopticon implements the graph layout algorithm used by Graphviz' DOT program[1].
The Sugiyama class exposes this functionality to QtQuick2.
The QML files that reside in res/.

References
==========

[1] K. Sugiyama, S. Tagawa, and M. Toda.
    “Methods for Visual Understanding of Hierarchical Systems”.
    IEEE Transactions on Systems, Man, and Cybernetics, 1981.

2014-8-2

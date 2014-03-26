#include <gtest/gtest.h>

#if defined(WIN32) || defined(_WINDOWS)
#include <windows.h>
#endif

using namespace testing;

int main(int argc, char **argv)
{
#if defined(WIN32) || defined(_WINDOWS)
	// prevent failed assertions from opening a popup
	SetErrorMode (SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX | SEM_NOOPENFILEERRORBOX);
#endif

	InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}

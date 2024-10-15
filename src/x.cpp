#include <cstdlib>

extern "C" void throws(void) {
  throw 42;
}

extern "C" void aborts(void) {
  std::abort();
}

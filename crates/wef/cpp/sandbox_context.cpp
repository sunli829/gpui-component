#include "include/cef_sandbox_mac.h"

extern "C" {

void* wef_sandbox_context_create(char* argv[], int argc) {
  CefScopedSandboxContext* ctx = new CefScopedSandboxContext();
  if (!sandbox_context.Initialize(argc, argv)) {
    delete library_loader;
    return nullptr;
  }
  return ctx;
}

void wef_sandbox_context_destroy(void* p) {
  CefScopedSandboxContext* ctx = static_cast<CefScopedSandboxContext*>(p);
  delete CefScopedSandboxContext;
}

}  // extern "C"
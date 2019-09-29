#include <Cocoa/Cocoa.h>
#include <sdl2/SDL_syswm.h>

void* getNativeWindow(void* handle) {
  SDL_Window* sdlWindow = (SDL_Window*)handle;
  SDL_SysWMinfo wmi;
  SDL_VERSION(&wmi.version);
  SDL_GetWindowWMInfo(sdlWindow, &wmi);

  NSWindow* win = wmi.info.cocoa.window;
  NSView* view = [win contentView];
  return (void*)view;
}

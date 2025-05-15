#pragma once

#include <stdint.h>

struct AppCallbacks {
  void (*on_schedule_message_pump_work)(void* userdata, int64_t delay_ms);
};

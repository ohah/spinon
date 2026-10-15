#include <errno.h>
#include <fcntl.h>
#include <linux/input.h>
#include <linux/uinput.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/ioctl.h>
#include <unistd.h>

static int device_fd = -1;
static int device_created = 0;
static volatile sig_atomic_t interrupted = 0;

static void on_signal(int signal_number) {
  (void)signal_number;
  interrupted = 1;
}

static void cleanup(void) {
  if (device_fd < 0) return;
  if (device_created) {
    ioctl(device_fd, UI_DEV_DESTROY);
    device_created = 0;
  }
  close(device_fd);
  device_fd = -1;
}

static void fail(const char *operation) {
  fprintf(stderr, "%s: %s\n", operation, strerror(errno));
  cleanup();
  exit(1);
}

static void enable_ioctl(unsigned long request, int value, const char *operation) {
  if (ioctl(device_fd, request, value) < 0) fail(operation);
}

static void set_abs_axis(int axis, int min, int max, int resolution) {
  enable_ioctl(UI_SET_ABSBIT, axis, "enable absolute axis");
  struct uinput_abs_setup setup = {
      .code = (unsigned short)axis,
      .absinfo = {
          .minimum = min,
          .maximum = max,
          .resolution = resolution,
      },
  };
  if (ioctl(device_fd, UI_ABS_SETUP, &setup) < 0) fail("configure absolute axis");
}

static void emit_event(unsigned short type, unsigned short code, int value) {
  struct input_event event;
  memset(&event, 0, sizeof(event));
  event.type = type;
  event.code = code;
  event.value = value;
  if (write(device_fd, &event, sizeof(event)) != sizeof(event)) fail("write input event");
}

static void sync_report(void) {
  emit_event(EV_SYN, SYN_REPORT, 0);
}

static void sleep_ms(unsigned milliseconds) {
  struct timespec remaining = {
    .tv_sec = milliseconds / 1000,
    .tv_nsec = (long)(milliseconds % 1000) * 1000000L,
  };
  while (!interrupted && nanosleep(&remaining, &remaining) < 0 && errno == EINTR) {}
}

static int parse_int(const char *value, const char *name, int min, int max) {
  char *end = NULL;
  errno = 0;
  long parsed = strtol(value, &end, 10);
  if (errno != 0 || end == value || *end != '\0' || parsed < min || parsed > max) {
    fprintf(stderr, "%s must be an integer in [%d,%d]\n", name, min, max);
    exit(2);
  }
  return (int)parsed;
}

static void set_slot(int slot, int tracking_id, int x, int y) {
  emit_event(EV_ABS, ABS_MT_SLOT, slot);
  emit_event(EV_ABS, ABS_MT_TRACKING_ID, tracking_id);
  if (tracking_id >= 0) {
    emit_event(EV_ABS, ABS_MT_POSITION_X, x);
    emit_event(EV_ABS, ABS_MT_POSITION_Y, y);
    emit_event(EV_ABS, ABS_MT_TOUCH_MAJOR, 8);
    emit_event(EV_ABS, ABS_MT_TOUCH_MINOR, 8);
    emit_event(EV_ABS, ABS_MT_PRESSURE, 40);
    emit_event(EV_ABS, ABS_MT_TOOL_TYPE, MT_TOOL_FINGER);
  }
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "usage: %s <device-inspection-delay-seconds 1..10>\n", argv[0]);
    return 2;
  }
  const unsigned delay_seconds = (unsigned)parse_int(argv[1], "delay", 1, 10);

  struct sigaction action;
  memset(&action, 0, sizeof(action));
  action.sa_handler = on_signal;
  sigemptyset(&action.sa_mask);
  sigaction(SIGINT, &action, NULL);
  sigaction(SIGTERM, &action, NULL);

  device_fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
  if (device_fd < 0) fail("open /dev/uinput");

  enable_ioctl(UI_SET_EVBIT, EV_KEY, "enable EV_KEY");
  enable_ioctl(UI_SET_EVBIT, EV_ABS, "enable EV_ABS");
  enable_ioctl(UI_SET_KEYBIT, BTN_TOUCH, "enable BTN_TOUCH");
  enable_ioctl(UI_SET_KEYBIT, BTN_TOOL_FINGER, "enable BTN_TOOL_FINGER");
  enable_ioctl(UI_SET_KEYBIT, BTN_TOOL_DOUBLETAP, "enable BTN_TOOL_DOUBLETAP");
  enable_ioctl(UI_SET_PROPBIT, INPUT_PROP_POINTER, "enable pointer property");
  enable_ioctl(UI_SET_PROPBIT, INPUT_PROP_BUTTONPAD, "enable buttonpad property");

  struct uinput_setup descriptor;
  memset(&descriptor, 0, sizeof(descriptor));
  snprintf(descriptor.name, UINPUT_MAX_NAME_SIZE, "Spinon C13.3 virtual touchpad probe");
  descriptor.id.bustype = BUS_VIRTUAL;
  descriptor.id.vendor = 0x5350;
  descriptor.id.product = 0xc13f;
  descriptor.id.version = 1;
  if (ioctl(device_fd, UI_DEV_SETUP, &descriptor) < 0) fail("configure device descriptor");

  set_abs_axis(ABS_MT_SLOT, 0, 1, 0);
  set_abs_axis(ABS_MT_TRACKING_ID, 0, 65535, 0);
  set_abs_axis(ABS_MT_POSITION_X, 0, 12000, 100);
  set_abs_axis(ABS_MT_POSITION_Y, 0, 8000, 100);
  set_abs_axis(ABS_MT_TOUCH_MAJOR, 0, 255, 1);
  set_abs_axis(ABS_MT_TOUCH_MINOR, 0, 255, 1);
  set_abs_axis(ABS_MT_PRESSURE, 0, 255, 1);
  set_abs_axis(ABS_MT_TOOL_TYPE, 0, MT_TOOL_MAX, 0);
  if (ioctl(device_fd, UI_DEV_CREATE) < 0) fail("create uinput device");
  device_created = 1;
  printf("device-created name=%s inspection_delay=%u coordinate_resolution_counts_per_mm=100\n",
         descriptor.name, delay_seconds);
  fflush(stdout);

  for (unsigned i = 0; i < delay_seconds * 10 && !interrupted; ++i) sleep_ms(100);

  const int first_x = 4500;
  const int second_x = 6500;
  const int start_y = 3500;
  set_slot(0, 1, first_x, start_y);
  emit_event(EV_KEY, BTN_TOOL_FINGER, 1);
  emit_event(EV_KEY, BTN_TOUCH, 1);
  sync_report();
  sleep_ms(10);
  set_slot(1, 2, second_x, start_y);
  emit_event(EV_KEY, BTN_TOOL_FINGER, 0);
  emit_event(EV_KEY, BTN_TOOL_DOUBLETAP, 1);
  sync_report();
  sleep_ms(15);

  for (int step = 1; step <= 6 && !interrupted; ++step) {
    const int y = start_y + step * 400;
    set_slot(0, 1, first_x, y);
    set_slot(1, 2, second_x, y);
    sync_report();
    sleep_ms(30);
  }
  sleep_ms(2000);

  set_slot(0, -1, 0, 0);
  set_slot(1, -1, 0, 0);
  emit_event(EV_KEY, BTN_TOOL_DOUBLETAP, 0);
  emit_event(EV_KEY, BTN_TOOL_FINGER, 0);
  emit_event(EV_KEY, BTN_TOUCH, 0);
  sync_report();
  cleanup();
  puts("device-destroyed=true");
  return interrupted ? 130 : 0;
}

#include <errno.h>
#include <fcntl.h>
#include <linux/input.h>
#include <linux/uinput.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <unistd.h>

static int device_fd = -1;
static int device_created = 0;

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

static void set_abs_axis(struct uinput_user_dev *descriptor, int axis, int min, int max) {
  enable_ioctl(UI_SET_ABSBIT, axis, "enable absolute axis");
  descriptor->absmin[axis] = min;
  descriptor->absmax[axis] = max;
  descriptor->absfuzz[axis] = 0;
  descriptor->absflat[axis] = 0;
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

int main(int argc, char **argv) {
  if (argc != 5) {
    fprintf(stderr,
        "usage: %s <relative-x> <relative-y> <h|v> <device-inspection-delay-seconds>\n",
        argv[0]);
    return 2;
  }

  const int relative_x = parse_int(argv[1], "relative-x", 1, 32767);
  const int relative_y = parse_int(argv[2], "relative-y", 1, 32767);
  const int delay_seconds = parse_int(argv[4], "delay", 1, 10);
  const int wheel_axis = strcmp(argv[3], "h") == 0 ? REL_HWHEEL
      : strcmp(argv[3], "v") == 0 ? REL_WHEEL : -1;
  if (wheel_axis < 0) {
    fprintf(stderr, "axis must be h or v\n");
    return 2;
  }

  device_fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
  if (device_fd < 0) fail("open /dev/uinput");

  enable_ioctl(UI_SET_EVBIT, EV_KEY, "enable EV_KEY");
  enable_ioctl(UI_SET_EVBIT, EV_REL, "enable EV_REL");
  enable_ioctl(UI_SET_EVBIT, EV_ABS, "enable EV_ABS");
  enable_ioctl(UI_SET_KEYBIT, BTN_LEFT, "enable BTN_LEFT");
  enable_ioctl(UI_SET_KEYBIT, BTN_TOUCH, "enable BTN_TOUCH");
  enable_ioctl(UI_SET_KEYBIT, BTN_TOOL_FINGER, "enable BTN_TOOL_FINGER");
  enable_ioctl(UI_SET_RELBIT, REL_X, "enable REL_X");
  enable_ioctl(UI_SET_RELBIT, REL_Y, "enable REL_Y");
  enable_ioctl(UI_SET_RELBIT, REL_WHEEL, "enable REL_WHEEL");
  enable_ioctl(UI_SET_RELBIT, REL_HWHEEL, "enable REL_HWHEEL");
  enable_ioctl(UI_SET_PROPBIT, INPUT_PROP_POINTER, "enable pointer property");
  enable_ioctl(UI_SET_PROPBIT, INPUT_PROP_BUTTONPAD, "enable buttonpad property");

  struct uinput_user_dev descriptor;
  memset(&descriptor, 0, sizeof(descriptor));
  snprintf(descriptor.name, UINPUT_MAX_NAME_SIZE,
      "Spinon C13.3 composite mouse touchpad probe");
  descriptor.id.bustype = BUS_VIRTUAL;
  descriptor.id.vendor = 0x5350;
  descriptor.id.product = 0xc134;
  descriptor.id.version = 1;
  set_abs_axis(&descriptor, ABS_X, 0, 32767);
  set_abs_axis(&descriptor, ABS_Y, 0, 32767);
  set_abs_axis(&descriptor, ABS_MT_SLOT, 0, 0);
  set_abs_axis(&descriptor, ABS_MT_TRACKING_ID, 0, 65535);
  set_abs_axis(&descriptor, ABS_MT_POSITION_X, 0, 32767);
  set_abs_axis(&descriptor, ABS_MT_POSITION_Y, 0, 32767);
  set_abs_axis(&descriptor, ABS_MT_TOUCH_MAJOR, 0, 255);
  set_abs_axis(&descriptor, ABS_MT_TOUCH_MINOR, 0, 255);
  set_abs_axis(&descriptor, ABS_MT_TOOL_TYPE, 0, MT_TOOL_MAX);

  if (write(device_fd, &descriptor, sizeof(descriptor)) != sizeof(descriptor)) {
    fail("write device descriptor");
  }
  if (ioctl(device_fd, UI_DEV_CREATE) < 0) fail("create uinput device");
  device_created = 1;
  printf("device-created name=%s cursor_delta=%d,%d axis=%s delay=%d\n",
      descriptor.name, relative_x, relative_y, argv[3], delay_seconds);
  fflush(stdout);
  sleep((unsigned)delay_seconds);

  emit_event(EV_REL, REL_X, -32768);
  emit_event(EV_REL, REL_Y, -32768);
  sync_report();
  usleep(150000);
  emit_event(EV_REL, REL_X, relative_x);
  emit_event(EV_REL, REL_Y, relative_y);
  sync_report();
  usleep(250000);
  emit_event(EV_REL, (unsigned short)wheel_axis, 1);
  sync_report();
  usleep(300000);

  cleanup();
  puts("device-destroyed=true");
  return 0;
}

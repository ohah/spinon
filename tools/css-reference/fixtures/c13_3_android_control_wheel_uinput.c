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

static int parse_control(const char *value) {
  if (strcmp(value, "left") == 0) return KEY_LEFTCTRL;
  if (strcmp(value, "right") == 0) return KEY_RIGHTCTRL;
  fprintf(stderr, "control must be left or right\n");
  exit(2);
}

static int parse_axis(const char *value) {
  if (strcmp(value, "horizontal") == 0) return REL_HWHEEL;
  if (strcmp(value, "vertical") == 0) return REL_WHEEL;
  fprintf(stderr, "axis must be horizontal or vertical\n");
  exit(2);
}

static int parse_coordinate(const char *value, const char *name) {
  char *end = NULL;
  errno = 0;
  long coordinate = strtol(value, &end, 10);
  if (errno != 0 || end == value || *end != '\0' || coordinate <= 0 || coordinate > 32767) {
    fprintf(stderr, "%s must be an integer in [1,32767]\n", name);
    exit(2);
  }
  return (int)coordinate;
}

int main(int argc, char **argv) {
  if (argc != 5) {
    fprintf(stderr, "usage: %s <left|right> <horizontal|vertical> <screen-x> <screen-y>\n", argv[0]);
    return 2;
  }

  const int control_key = parse_control(argv[1]);
  const int wheel_axis = parse_axis(argv[2]);
  const int screen_x = parse_coordinate(argv[3], "screen-x");
  const int screen_y = parse_coordinate(argv[4], "screen-y");

  device_fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
  if (device_fd < 0) fail("open /dev/uinput");
  enable_ioctl(UI_SET_EVBIT, EV_KEY, "enable EV_KEY");
  enable_ioctl(UI_SET_EVBIT, EV_REL, "enable EV_REL");
  enable_ioctl(UI_SET_KEYBIT, BTN_LEFT, "enable BTN_LEFT");
  enable_ioctl(UI_SET_KEYBIT, control_key, "enable Control key");
  enable_ioctl(UI_SET_RELBIT, REL_X, "enable REL_X");
  enable_ioctl(UI_SET_RELBIT, REL_Y, "enable REL_Y");
  enable_ioctl(UI_SET_RELBIT, REL_WHEEL, "enable REL_WHEEL");
  enable_ioctl(UI_SET_RELBIT, REL_HWHEEL, "enable REL_HWHEEL");
  enable_ioctl(UI_SET_PROPBIT, INPUT_PROP_POINTER, "enable pointer property");

  struct uinput_user_dev descriptor;
  memset(&descriptor, 0, sizeof(descriptor));
  snprintf(descriptor.name, UINPUT_MAX_NAME_SIZE, "Spinon C13.3 virtual wheel probe");
  descriptor.id.bustype = BUS_VIRTUAL;
  descriptor.id.vendor = 0x5350;
  descriptor.id.product = 0xc133;
  descriptor.id.version = 1;
  if (write(device_fd, &descriptor, sizeof(descriptor)) != sizeof(descriptor)) {
    fail("write uinput descriptor");
  }
  if (ioctl(device_fd, UI_DEV_CREATE) < 0) fail("create uinput device");
  device_created = 1;
  printf("device-created control=%s axis=%s screen=%d,%d\n", argv[1], argv[2], screen_x, screen_y);
  fflush(stdout);
  usleep(800000);

  emit_event(EV_REL, REL_X, -32768);
  emit_event(EV_REL, REL_Y, -32768);
  sync_report();
  usleep(150000);
  emit_event(EV_REL, REL_X, screen_x);
  emit_event(EV_REL, REL_Y, screen_y);
  sync_report();
  usleep(200000);
  emit_event(EV_KEY, control_key, 1);
  sync_report();
  usleep(200000);
  emit_event(EV_REL, wheel_axis, 1);
  sync_report();
  usleep(200000);
  emit_event(EV_KEY, control_key, 0);
  sync_report();
  usleep(200000);

  if (ioctl(device_fd, UI_DEV_DESTROY) < 0) fail("destroy uinput device");
  device_created = 0;
  close(device_fd);
  device_fd = -1;
  puts("device-destroyed control-released=true");
  return 0;
}

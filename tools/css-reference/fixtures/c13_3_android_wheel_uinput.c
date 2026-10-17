#include <errno.h>
#include <fcntl.h>
#include <linux/input.h>
#include <linux/uinput.h>
#include <signal.h>
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

static void handle_signal(int signal_number) {
  cleanup();
  _exit(128 + signal_number);
}

static void fail(const char *operation) {
  fprintf(stderr, "%s: %s\n", operation, strerror(errno));
  cleanup();
  exit(1);
}

static void enable_ioctl(unsigned long request, int value, const char *operation) {
  if (ioctl(device_fd, request, value) < 0) fail(operation);
}

static int parse_bounded_integer(const char *value, int minimum, int maximum,
    const char *name) {
  char *end = NULL;
  errno = 0;
  long parsed = strtol(value, &end, 10);
  if (errno != 0 || end == value || *end != '\0'
      || parsed < minimum || parsed > maximum) {
    fprintf(stderr, "%s must be an integer in [%d,%d]\n", name, minimum, maximum);
    exit(2);
  }
  return (int)parsed;
}

static int parse_control_key(const char *value) {
  if (strcmp(value, "none") == 0) return 0;
  if (strcmp(value, "left") == 0) return KEY_LEFTCTRL;
  if (strcmp(value, "right") == 0) return KEY_RIGHTCTRL;
  fprintf(stderr, "control must be none, left, or right\n");
  exit(2);
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

int main(int argc, char **argv) {
  if (argc != 6) {
    fprintf(stderr, "usage: %s <screen-x> <screen-y> <hscroll> <vscroll> <none|left|right>\n", argv[0]);
    return 2;
  }

  const int screen_x = parse_bounded_integer(argv[1], 1, 32767, "screen-x");
  const int screen_y = parse_bounded_integer(argv[2], 1, 32767, "screen-y");
  const int horizontal = parse_bounded_integer(argv[3], -1, 1, "hscroll");
  const int vertical = parse_bounded_integer(argv[4], -1, 1, "vscroll");
  const int control_key = parse_control_key(argv[5]);
  if (horizontal == 0 && vertical == 0) {
    fprintf(stderr, "at least one wheel axis must be non-zero\n");
    return 2;
  }

  signal(SIGINT, handle_signal);
  signal(SIGTERM, handle_signal);
  device_fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
  if (device_fd < 0) fail("open /dev/uinput");
  enable_ioctl(UI_SET_EVBIT, EV_KEY, "enable EV_KEY");
  enable_ioctl(UI_SET_EVBIT, EV_REL, "enable EV_REL");
  enable_ioctl(UI_SET_KEYBIT, BTN_LEFT, "enable BTN_LEFT");
  enable_ioctl(UI_SET_KEYBIT, KEY_LEFTCTRL, "enable left Control");
  enable_ioctl(UI_SET_KEYBIT, KEY_RIGHTCTRL, "enable right Control");
  enable_ioctl(UI_SET_RELBIT, REL_X, "enable REL_X");
  enable_ioctl(UI_SET_RELBIT, REL_Y, "enable REL_Y");
  enable_ioctl(UI_SET_RELBIT, REL_WHEEL, "enable REL_WHEEL");
  enable_ioctl(UI_SET_RELBIT, REL_HWHEEL, "enable REL_HWHEEL");
  enable_ioctl(UI_SET_PROPBIT, INPUT_PROP_POINTER, "enable pointer property");

  struct uinput_user_dev descriptor;
  memset(&descriptor, 0, sizeof(descriptor));
  snprintf(descriptor.name, UINPUT_MAX_NAME_SIZE, "Spinon C13.3 API29 wheel matrix");
  descriptor.id.bustype = BUS_VIRTUAL;
  descriptor.id.vendor = 0x5350;
  descriptor.id.product = 0xc133;
  descriptor.id.version = 1;
  if (write(device_fd, &descriptor, sizeof(descriptor)) != sizeof(descriptor)) {
    fail("write uinput descriptor");
  }
  if (ioctl(device_fd, UI_DEV_CREATE) < 0) fail("create uinput device");
  device_created = 1;
  printf("device-created screen=%d,%d hscroll=%d vscroll=%d control=%s\n",
      screen_x, screen_y, horizontal, vertical, argv[5]);
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

  if (control_key != 0) {
    emit_event(EV_KEY, (unsigned short)control_key, 1);
    sync_report();
    usleep(150000);
  }
  if (horizontal != 0) emit_event(EV_REL, REL_HWHEEL, horizontal);
  if (vertical != 0) emit_event(EV_REL, REL_WHEEL, vertical);
  sync_report();
  usleep(200000);
  if (control_key != 0) {
    emit_event(EV_KEY, (unsigned short)control_key, 0);
    sync_report();
    usleep(150000);
  }

  if (ioctl(device_fd, UI_DEV_DESTROY) < 0) fail("destroy uinput device");
  device_created = 0;
  close(device_fd);
  device_fd = -1;
  puts("device-destroyed");
  return 0;
}

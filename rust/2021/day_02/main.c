#include <stdio.h>
#include <stdint.h>
#include <string.h>

#define streq(a,b) (strcmp((a),(b))==0)

int main(void) {
  char direction[10];
  uint64_t value = 0, x = 0, y = 0, z = 0;

  for (int i = 0; i < 1000; i++) {
    scanf("%s %llu", direction, &value);
    if (streq(direction, "forward")) {
      x += value;
      y += (z * value);
    } else if (streq(direction, "down")) {
      z += value;
    } else if (streq(direction, "up")) {
      z -= value;
    }
  }

  printf("%llu", x * y);
}

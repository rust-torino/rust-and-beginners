#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char *
present(char * name) {
  static char message[16];
  snprintf(message, 16, "I am %s", name);
  return message;
}

char *
hello_world(char * message, char * name) {
  char * present_message = present(name);

  // Size of "Hello world! " = 13
  // Additional 1 is for space
  // Additional 1 is for the string terminator
  size_t const message_len = strlen(present_message) + 15 + strlen(message);

  char * out_message = malloc(message_len);
  sprintf(out_message, "Hello world! %s %s", message, present_message);
  return out_message;
}

int main(int argc, char* argv[]) {
  char * yell = "Oh yeah!";
  char * message = hello_world(yell, argv[1]);
  printf("%s\n", message);
  free(message);
}


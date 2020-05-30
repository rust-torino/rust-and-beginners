#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char const *
present(char const * const name) {
  static char message[16];
  int written = snprintf(message, 16, "I am %s", name);

  if (written < 0) {
    return NULL;
  } else {
    return message;
  }
}

char *
hello_world(char const * const message, char const * const name) {
  char const * const present_message = present(name);

  if (present_message == NULL) {
    return NULL;
  }

  // Size of "Hello world! " = 13
  // Additional 1 is for the string terminator
  size_t message_len = strlen(present_message) + 14;

  char * out_message;
  if (message != NULL) {
    // 1 is for an additional space
    message_len += 1 + strlen(message);
  }

  out_message = malloc(sizeof(char) * message_len);
  size_t written;
  if (message == NULL) {
    written = sprintf(out_message, "Hello world! %s", present_message);
  } else {
    written = sprintf(out_message, "Hello world! %s %s", message, present_message);
  }

  if (written < 0) {
    free(out_message);
    return NULL;
  } else {
    return out_message;
  }
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    return 0;
  }

  char const * const yell = "Oh yeah!";
  char * message = hello_world(yell, argv[1]);
  if (message != NULL) {
    printf("%s\n", message);
    free(message);
  }
}

CC ?= cc
CFLAGS := -O2 -Wall
SRCS = main.c greet.c
UNAME != uname -s
OBJS = $(SRCS:.c=.o)

include config.mk

define link
$(CC) -o $@ $^
endef

.PHONY: all clean

all: greeter

greeter: $(OBJS)
	$(call link)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f greeter $(OBJS)

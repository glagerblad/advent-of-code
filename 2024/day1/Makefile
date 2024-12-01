CC = gcc
CFLAGS = -Wall -Werror -O2 -lm
TARGET = main
SRC = main.c

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC)

run: $(TARGET)
	./$(TARGET)

clean: 
	rm -f $(TARGET)

.PHONY: all run clean

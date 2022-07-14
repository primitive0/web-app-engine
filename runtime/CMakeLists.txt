cmake_minimum_required(VERSION 3.22)
project(http_lang C)

set(CMAKE_C_STANDARD 11)

add_executable(http_lang src/main.c src/string.h src/mem.h src/dyn.h src/printer.h src/strconv.h)

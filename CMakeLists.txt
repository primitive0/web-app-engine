cmake_minimum_required(VERSION 3.22)
project(http_lang C)

set(CMAKE_C_STANDARD 11)

add_executable(http_lang main.c string.h mem.h dyn.h printer.h strconv.h)

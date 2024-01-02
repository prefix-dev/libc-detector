/*
 * Taken from https://github.com/njsmith/posy/blob/61ff56c68a6aeff744b274170c32f59ac9ac20e1/src/platform_tags/linux-glibc-detectors/glibc-detector.c
 * Licensed under MIT/APACHE-2.0

 * A tiny C program that tries to fetch the version of glibc that it's run
 * against.
 */

#include <gnu/libc-version.h>
#include <stdio.h>

int main(int argc, char** argv)
{
    puts(gnu_get_libc_version());
    return 0;
}

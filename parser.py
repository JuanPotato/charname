#!/usr/bin/env python3

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import sys

def get_data(line):
    data = line.split(';')
    num = int(data[0], 16)
    name = data[1]
    if name == '<control>' and data[10] != '':
        name = data[10]

    return (num, name)


def main(filename, dstfilename):
    with open(filename, 'r') as f:
        data = [get_data(line.strip()) for line in f]
    
    with open(dstfilename, 'w') as f:
        f.write('\n'.join(f'{num:X};{name}' for (num, name) in data))


if __name__ == '__main__':
    src = 'UnicodeData.txt'
    dst = 'UnicodeDataFixed.txt'

    if len(sys.argv) == 2:
        src = sys.argv[1]
    if len(sys.argv) == 3:
        src = sys.argv[1]
        dst = sys.argv[2]
    else:
        print(f'Heck, use it like this.\n {sys.argv[0]} sourcefile.txt destfile.txt');
        exit(1)

    main(src, dst)

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
    with open(filename) as fin, open(dstfilename, 'w') as fout:
        for line in fin:
            num, name = get_data(line.strip())
            fout.write(f'{num:X};{name}\n')


if __name__ == '__main__':
    # https://www.unicode.org/Public/13.0.0/ucd/UnicodeData.txt
    src = 'UnicodeData.txt'
    dst = 'UnicodeDataFixed.txt'

    if len(sys.argv) == 1:
        # ya good
        pass
    elif len(sys.argv) == 2:
        src = sys.argv[1]
    elif len(sys.argv) == 3:
        src = sys.argv[1]
        dst = sys.argv[2]
    else:
        print(f'Heck, use it like this.\n {sys.argv[0]} sourcefile.txt destfile.txt');
        exit(1)

    main(src, dst)

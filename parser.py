#!/usr/bin/env python3

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import re
import sys
import os.path
import requests

GET_NAME_CHECKED_TEMPLATE = \
'''
pub fn get_name_checked(c: u32) -> Option<&'static str> {{
    Some(match UNICODE.get(&c) {{
        Some(s) => s,

        None => match c {{
{range_matches}
            _ => return None,
        }},
    }})
}}
'''

RANGE_MATCH_TEMPLATE = '            0x{first:X}..=0x{last:X} => "{name}",\n'


def get_data(line):
    data = line.split(';')
    num = int(data[0], 16)
    name = data[1]
    if name == '<control>' and data[10] != '':
        name = data[10]

    return (num, name)


def main(filename, dstfilename):
    char_ranges_firsts = {}
    char_ranges_lasts = {}

    with open(filename) as fin, open(dstfilename, 'w') as fout:
        for line in fin:
            num, name = get_data(line.strip())
            fout.write(f'{num:X};{name}\n')

            if match := re.search(r'<([\w\s]+), (First|Last)>', name):
                range_name = match.group(1)

                if match.group(2) == 'First':
                    char_ranges_firsts[range_name] = num
                else:
                    char_ranges_lasts[range_name] = num
    
    range_matches = ''
    for range_name in char_ranges_firsts:
        first = char_ranges_firsts[range_name]
        last = char_ranges_lasts[range_name]
        range_matches += RANGE_MATCH_TEMPLATE.format(first=first, last=last, name=range_name)

    print(GET_NAME_CHECKED_TEMPLATE.format(range_matches=range_matches))


if __name__ == '__main__':
    latest = "https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt"
    src = 'UnicodeData.txt'
    dst = 'UnicodeDataFixed.txt'

    if len(sys.argv) == 1:
        if not os.path.exists(src):
            print(f'Downloading {src} from {latest}')
            r = requests.get(latest)
            with open(src, 'w+') as src_f:
                src_f.write(r.text)

    elif len(sys.argv) == 2:
        src = sys.argv[1]

    elif len(sys.argv) == 3:
        src = sys.argv[1]
        dst = sys.argv[2]

    else:
        print(f'Heck, use it like this.\n {sys.argv[0]} sourcefile.txt destfile.txt');
        exit(1)

    main(src, dst)

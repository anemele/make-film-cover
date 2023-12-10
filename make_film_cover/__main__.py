""" Make film directory displayed with a cover. """

import argparse
import glob
import os.path
import sys
from itertools import chain

from .consts import FILTER_DB, FILTER_ENCODING
from .core import make_cover
from .log import logger

parser = argparse.ArgumentParser(
    prog=__package__ if len(sys.argv) == 1 else sys.argv[1], description=__doc__
)
parser.add_argument('film', nargs='+', type=str, help='Film path')
parser.add_argument(
    '--debug',
    action='store_true',
    help='DEBUG mode, ignore the filter file.',
)
parser.add_argument(
    '--ignore',
    action='store_true',
    help='Do not read but write the filter file.',
)
parser.add_argument(
    '--rebuild',
    action='store_true',
    help='Rebuild the filter file.',
)
args = parser.parse_args(sys.argv[2:])


args_film: list[str] = args.film
args_debug: bool = args.debug
args_ignore: bool = args.ignore
args_rebuild: bool = args.rebuild

# 以下逻辑不好写
""" 梳理逻辑：
程序提供 3 个参数，都是 bool 值，作用都是与过滤文件相关。
（过滤文件是避免已经制作好的目录被重复制作）
--debug 是调试模式，本来是为了测试功能，正好用于跳过过滤文件。
--ignore 是忽略过滤文件的过滤作用，但是会向其中添加新内容。
--rebuild 是重建过滤文件。还有一种情况，如果过滤文件不存在，则也可走重建。
"""

films = filter(os.path.isdir, chain.from_iterable(map(glob.iglob, args_film)))

if args_debug:
    for film in films:
        logger.info(f'debug: {make_cover(film)}')
    exit()

film_dict = {os.path.basename(file): file for file in films}
film_set = set(film_dict.keys())
if not FILTER_DB.exists() or args_rebuild:
    filter_set = set()
else:
    tmp = FILTER_DB.read_text(encoding=FILTER_ENCODING).strip().split('\n')
    filter_set = set(tmp)
    if not args_ignore:
        for film in film_set & filter_set:
            logger.info(f'skip: {film}')
        film_set = film_set - filter_set

for film in film_set:
    result = make_cover(film_dict[film])
    if result is not None:
        logger.info(f'done: {result}')
        filter_set.add(film)

FILTER_DB.write_text('\n'.join(sorted(filter_set)), encoding=FILTER_ENCODING)

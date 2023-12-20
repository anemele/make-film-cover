import glob
import os.path
from typing import Optional

import win32api
import win32con
from PIL import Image, UnidentifiedImageError

from .consts import COVER_PATH, ICON_PATH, ICON_SIZE, ICON_SIZE_TUPLE
from .log import logger


def square_image(file: str) -> Image.Image:
    input_image = Image.open(file)
    m = ICON_SIZE

    if (rate := m / max(input_image.size)) < 1:
        w, h = input_image.size
        size = (round(w * rate), round(h * rate))
        input_image = input_image.resize(size)

    w, h = input_image.size
    padding_w = (m - w) >> 1
    padding_h = (m - h) >> 1

    new_image = Image.new('RGBA', (m, m))
    # 思路一，逐个复制像素（效率太低）
    # for i in range(w):
    #     for j in range(h):
    #         new_image.putpixel(
    #             (padding_w + i, padding_h + j), input_image.getpixel((i, j))
    #         )
    # 思路二，粘贴图片（思路早有了，方法查了好久）
    new_image.paste(input_image, (padding_w, padding_h))

    return new_image


def make_icon(film_path: str) -> None:
    match = glob.iglob(f'{film_path}/{COVER_PATH}.*')
    # 此处尝试每一个匹配，如果成功则结束，否则认为不存在，抛出 StopIteration
    for img_path in match:
        try:
            sqr_img = square_image(img_path)
        except UnidentifiedImageError:
            continue
        break
    else:
        raise StopIteration

    ico_path = os.path.join(film_path, f'{ICON_PATH}.ico')
    sqr_img.save(ico_path, sizes=ICON_SIZE_TUPLE)
    # 设置隐藏属性
    # win32api.SetFileAttributes(ico_path, win32con.FILE_ATTRIBUTE_HIDDEN)


def write_ini(film_path: str) -> None:
    desktop_ini = os.path.join(film_path, 'desktop.ini')
    # 如果已经存在，要设为普通文件，否则无法编辑
    if os.path.exists(desktop_ini):
        win32api.SetFileAttributes(desktop_ini, win32con.FILE_ATTRIBUTE_NORMAL)
    with open(desktop_ini, 'w') as fp:
        fp.write(f'[.ShellClassInfo]\nIconResource={ICON_PATH}.ico,0\n')
    # 设置系统属性和隐藏属性
    win32api.SetFileAttributes(
        desktop_ini, win32con.FILE_ATTRIBUTE_HIDDEN | win32con.FILE_ATTRIBUTE_SYSTEM
    )


def make_cover(film_path: str) -> Optional[str]:
    # step 1: icon file
    try:
        make_icon(film_path)
    except StopIteration:
        logger.error(f'no {COVER_PATH}.* file found in {film_path}')
        return
    # step 2: desktop.ini
    write_ini(film_path)
    # step 3: attributes
    win32api.SetFileAttributes(film_path, win32con.FILE_ATTRIBUTE_READONLY)

    return film_path

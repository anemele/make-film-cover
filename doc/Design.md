# Design for this program

This is a command line interface program named `mfc`,
it requires a positional argument `path`,
which is the folder path,
and an optional argument `name`,
which is the image name, `cover.jpg` by default,
then the image file path is `path\name`.

There are 3 steps to finish the task

1. image processing
2. `desktop.ini` file setting
3. folder setting

**image**

The image file is usually a jpg or png
with a rectangle shape.

The first thing is to create a squared image with
apparent pixels filled the shorter edge.
For example, the original image is a 30x40 one,
then the squared image is a 40x40 one, which filled with
apparent pixels from 1-5 and 36-40.

Then convert this squared image as icon image with size 256,
which is the maximum supported size.
Save the icon image as a file,
make it `hidden` (optional).

**`desktop.ini`**

The key of this program is in the system file `desktop.ini`.

Create this file in the folder,
write the following content,
make it `system` and `hidden`.

```ini
[.ShellClassInfo]
IconResource=cover.ico,0
```

**folder**

make the folder `readonly`.

---

I think the cli should accept only one argument `path`,
set the image file as the same name `cover.jpg` or `cover.png` or
other format.

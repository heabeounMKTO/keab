# keab
a very smol multi-threaded image compression utility written in rust , using `turbojpeg` and `image-rs` library 

### why

am lazy 

### usage

download the binaries from the [releases](https://github.com/heabeounMKTO/keab/releases) page

```shell
./keab --folder path/to/image/folder --quality 10 --subsamp 422
```
*note: this program will run on all threads*

|options|what is it|
|---|---|
|`folder`|the folder with the pictures you want to compress|
|`quality`|compression quality|
|`subsamp`|chroma subsampling i.e 422 for 4:2:2|

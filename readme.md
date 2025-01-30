**Version: 0.0.2-e**

Note: Letters after the version numbers are their sub-evolutions when non-major changes were added. Version number will increase after the letter hits `z`.
If we made a push and realized there is a bug or mistake in the code that is lethal we will delete the Docker tag and push the fixes under the same version tag.

# Saihex Studios' Multipurpose Media Server
<img align="right" width="128" src="https://img.saihex.com/software_logos/multipurpose_media_server.svg">

This software is used by [Saihex Studios](www.saihex.com) as the web server for allowing access to our image server. Back then we used Nginx for this purpose but for storage concern we made this software that can scale down images so that the server only have to hold a single image at high resolution and the program will scale it down depending on the query provided by the HTTP request.

Used dependencies
```
async-stream = "0.3.5" --  MIT
actix-web = "4" --  MIT OR Apache-2.0 
actix-files = "0.6" --  MIT OR Apache-2.0 
image = "0.25.1" --  MIT OR Apache-2.0 
tokio = { version = "1", features = ["full"] } --  MIT
mime_guess = "2.0" --  MIT
reqwest = { version = "0.12.4", features = ["blocking", "json"] } --  MIT OR Apache-2.0 
resvg = "0.41.0" --  MPL-2.0
webp = "0.3.0" -- MIT or Apache-2.0
```

Included fonts for on-the-fly SVG render feature

- [Audiowide](https://fonts.google.com/specimen/Audiowide) > [Open Font](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL)

**Docker Image**
```
saihex/multipurpose-media-server:v0.0.2-e
```

## General features
Works as normal public media server, on-the-fly downscaling feature for images, on-the-fly SVG to PNG conversion, and root path `index.html` redirect.

## How to use
In order to use this, you will need a Docker Daemon installed on your machine. Then, clone this git repository and build the Docker image. Once that's done, you may run it with the following configuration:

- Expose port `8080`
- Attach `/collection` volume to the directory that contains all the images and files you want to expose.

The downscaling feature works by providing certain query parameters to the HTTP GET request with key of "downscale":

- `m` for medium, this will downscale the image by half of its original resolution
- `s` for small, this will downscale the image by 1/4 of its original resolution, to those people out there: **HALF WHICH IS 1/2 IS __NOT__ LESS THAN 1/4**
- `fm` for forced-medium, this will downscale the image to 512 horizontally even if the original resolution is lower which will cause upscale instead.
- `fs` for forced-small, this will downscale the image to 256 horizontally even if the original resolution is lower which will cause upscale instead.

**Note: all image manipulation are programmed to maintain aspect ratio even forced ones.**

Empty path (`https://img.example.com/`) will make the program to load `index.html` in the collection directory. Will cause error 500 if none present in the directory.
Only `jpg`, `jpeg` and `png` is supported by the downscale feature. Any other files or invalid downscale value will cause the program to just respond with the original file.

## On the fly SVG to PNG conversion (0.0.2 and higher)
link to `https://img.example.com/svg_png` will run SVG-PNG mode. Query parameter with key of `src` must be given and contains value of the SVG file path relative to the collection directory shown as the closured text `https://img.example.com/(path/to/your/svg.svg)`. The program will request to itself on port `8080` to that file to read the SVG bytes then render it as PNG and return it as the body with content type `image/png`.

`scale` query parameter can be used to dictate the scale of the rendered PNG. By default the program will render the SVG at `512` vertical resolution and whatever resolution that follows the SVG canvas aspect ratio. Setting `scale` query parameter to `s` or `fs` will lower the vertical resolution to `128`.

*Side Note: Thanks Discord for making me had to make this for my website SVG embed image to work on your app*

## On the fly SVG/PNG/JPEG to WebP conversion (0.0.2-c and higher)
link to `https://img.example.com/webp`. Query parameter with key of `src` must be given and contains value of the image path relative to collection directory.
`scale` parameter can be used to downscale the images. This feature was added to further improve Saihex's website performance.

**Example:** `http://localhost:8080/webp?src=test.png&scale=s`

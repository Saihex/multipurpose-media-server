# Saihex Studios' Image Server Access Software
This software is used by [Saihex Studios](www.saihex.com) as the web server for allowing access to our image server. Back then we used Nginx for this purpose but for storage concern we made this software that can scale down images so that the server only have to hold a single image at high resolution and the program will scale it down depending on the query provided by the HTTP request.

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
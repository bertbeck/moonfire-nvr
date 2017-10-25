# Introduction

Moonfire NVR is an open-source security camera network video recorder, started
by Scott Lamb &lt;<slamb@slamb.org>&gt;. It saves H.264-over-RTSP streams from
IP cameras to disk into a hybrid format: video frames in a directory on
spinning disk, other data in a SQLite3 database on flash. It can construct
`.mp4` files for arbitrary time ranges on-the-fly. It does not decode,
analyze, or re-encode video frames, so it requires little CPU. It handles six
1080p/30fps streams on a [Raspberry Pi
2](https://www.raspberrypi.org/products/raspberry-pi-2-model-b/), using
less than 10% of the machine's total CPU.

So far, the web interface is basic: a filterable list of video segments,
with support for trimming them to arbitrary time ranges. No scrub bar yet.
There's also no support for motion detection, no authentication, and no config
UI.

![screenshot](screenshot.png)

<b>This branch is version 0.2 and is in development.</b>

Until version 1.0, there will be no
compatibility guarantees: configuration and storage formats may change from
version to version. There is an [upgrade procedure](guide/schema.md) but it is
not for the faint of heart.

<b>Version 0.2 Goals:</b>

Create an api that allows either a single jpeg or 1 frame mpeg4 to be requested for a camera (or optionally all cameras)

<b>Long term goals:</b>

Add features such as salient motion detection, live monitoring, bandwidth conservation. 

We welcome help; see [Getting help and getting
involved](#help) below. There are many exciting techniques we could use to
make this possible:

* avoiding CPU-intensive H.264 encoding in favor of simply continuing to use the
  camera's already-encoded video streams. Cheap IP cameras these days provide
  pre-encoded H.264 streams in both "main" (full-sized) and "sub" (lower
  resolution, compression quality, and/or frame rate) varieties. The "sub"
  stream is more suitable for fast computer vision work as well as
  remote/mobile streaming. Disk space these days is quite cheap (with 3 TB
  drives costing about $100), so we can afford to keep many camera-months of
  both streams on disk.
* decoding and analyzing only select "key" video frames (see
  [wikipedia](https://en.wikipedia.org/wiki/Video_compression_picture_types).
* off-loading expensive work to a GPU. Even the Raspberry Pi has a
  surprisingly powerful GPU.
* using [HTTP Live Streaming](https://en.wikipedia.org/wiki/HTTP_Live_Streaming)
  rather than requiring custom browser plug-ins.
* taking advantage of cameras' built-in motion detection. This is
  the most obvious way to reduce motion detection CPU. It's a last resort
  because these cheap cameras' proprietary algorithms are awful compared to
  those described on [changedetection.net](http://changedetection.net).
  Cameras have high false-positive and false-negative rates, are hard to
  experiment with (as opposed to rerunning against saved video files), and
  don't provide any information beyond if motion exceeded the threshold or
  not.

# Documentation

*   [License](LICENSE.txt) — GPLv3
*   [Building and installing](guide/install.md)
*   [Troubleshooting](guide/troubleshooting.md)

# <a name="help"></a> Getting help and getting involved

Please email the
[moonfire-nvr-users](https://groups.google.com/d/forum/moonfire-nvr-users)
mailing list with questions, or just to say you love/hate the software and
why. You can also file bugs and feature requests on the
[github issue tracker](https://github.com/scottlamb/moonfire-nvr/issues).

We welcome help with testing, development (in Rust, JavaScript, and HTML),
user interface/graphic design, and documentation. Please email the mailing
list if interested. Pull requests are welcome, but I encourage you to discuss
large changes on the mailing list or in a github issue first to save effort.

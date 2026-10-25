# ym-dl

ym-dl written in Rust with lossless support.
[Pre-compiled binaries](https://github.com/Sorrow446/Yandex-Music-Downloader/releases)

## Setup
Copy `config.toml.example` to `config.toml` (in the working directory or next to the executable):
```sh
# Windows (PowerShell)
Copy-Item config.toml.example config.toml

# Linux / macOS
cp config.toml.example config.toml
```

Open `config.toml` and set your `token`. You can also configure any other settings:

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `token` | string | `""` | Yandex Music OAuth token (required unless passed via CLI). |
| `format` | integer | `4` | Track download quality: `1` = AAC 64, `2` = AAC 192, `3` = AAC 256 / MP3 320, `4` = FLAC. |
| `out_path` | string | `"downloads"` | Output folder for downloads. Created automatically if missing. |
| `ffmpeg_path` | string | `""` | Custom path to FFmpeg binary (optional, auto-detected from PATH or exe folder). |
| `keep_covers` | boolean | `true` | Save album cover as `folder.jpg` in the album directory. |
| `write_covers` | boolean | `true` | Embed album artwork directly into audio metadata tags. |
| `get_original_covers` | boolean | `false` | Download full original resolution covers instead of 1000x1000. |
| `write_lyrics` | boolean | `true` | Save timed lyrics (`.lrc`) when available. |
| `sleep` | boolean | `false` | Pause between processing items to reduce rate-limiting risk. |
| `album_template` | string | `"{album_artist} - {album_title}"` | Album directory naming template. Variables: `{album_artist}`, `{album_title}`, `{label}`, `{year}`. |
| `track_template` | string | `"{track_num_pad}. {title}"` | Track filename naming template. Variables: `{artist}`, `{title}`, `{track_num}`, `{track_num_pad}`. |
## Token Acquisition
**Plus subscription required.**    

https://yandex-music.readthedocs.io/en/main/token.html

## Supported Media
Wrap any URLs that contain params in double quotes if running on Windows.

|Type|URL example|
| --- | --- |
|Album|`https://music.yandex.ru/album/33134482`
|Artist albums|`https://music.yandex.ru/artist/9838127`, `https://music.yandex.ru/artist/9838127/albums`
|Track|`https://music.yandex.ru/album/2955514/track/25128596`
|User playlist|`https://music.yandex.ru/users/user@gmail.com/playlists/1000`
|User favourites|`https://music.yandex.ru/users/user@gmail.com/playlists/3`

Other users' favourites and playlists are also supported, but they must be set to public.

## Usage
Args take priority over the config file.

Download two albums:   
`ym-dl.exe -u https://music.yandex.ru/album/33134482 https://music.yandex.ru/album/33199228`

Download a single album and from a text file containing links:   
`ym-dl.exe -u https://music.yandex.ru/album/33134482 G:\1.txt`

```
Usage: ym-dl.exe [OPTIONS] --urls <URLS>...

Options:
  -t, --token <TOKEN>                    OAuth token (overrides config.toml)
  -f, --format <FORMAT>                  1 = AAC 64, 2 = AAC 192, 3 = AAC 256 / MP3 320, 4 = FLAC
  -g, --get-original-covers              Download original covers instead of 1000x1000
  -k, --keep-covers                      Keep folder.jpg cover in album folder
  -o, --out-path <OUT_PATH>              Output path
  -s, --sleep                            Pause between processing to prevent rate limits
      --write-covers                     Write covers to tracks
      --write-lyrics                     Write timed lyrics (.lrc) when available
      --album-template <ALBUM_TEMPLATE>  Album directory template
      --track-template <TRACK_TEMPLATE>  Track filename template
  -u, --urls <URLS>...                   URLs or path to .txt file with URLs
  -h, --help                             Print help
  -V, --version                          Print version
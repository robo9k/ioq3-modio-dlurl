This experimental HTTP server produces mod.io download URLs that can be used with the built-in downloader of ioQuake3 games if mods are uploaded as .pk3.zip modfiles directly

The game needs to be configured [General settings → Content settings → Community settings → Mods](https://test.mod.io/g/<game_name_id>/admin/settings#content):  
`[x] Allow downloads for unauthenticated users`  
Otherwise download URLs are generated successfully, but trying to download them with ioq3 yields HTTP 401 with mod.io error 11060 `MODIO_INVALID_TOKEN`

With `[x] Allow direct downloads` the URLs are valid for longer, but that is not required.

```console
$ sudo apt-get install ioquake3
$ IOQ3=/usr/lib/ioquake3/ioquake3
$ IOQ3D=/usr/lib/ioquake3/ioq3ded
```

```console
$ cargo run
Listening on: 0.0.0.0:3000
```

```console
$ SRV_HOMEPATH=$(mktemp --directory)
$ SRV_BASEGAME="${SRV_HOMEPATH}/baseq3"
$ mkdir --parents "${SRV_BASEGAME}"

$ curl --location --output "${SRV_BASEGAME}/example.pk3" http://localhost:3000/baseq3/example.pk3
```

```console
$ pk3sum ~/.q3a/baseq3/example.pk3
123456789
```

```console
$ ${IOQ3D} +set sv_allowDownload 4 +sets sv_dlURL '"http://localhost:3000"' +set fs_homepath "${SRV_HOMEPATH}" +map example
]sv_referencedPak
    sv_referencedPakNames = "baseq3/example baseq3/pak8 baseq3/pak4"
    sv_referencedPaks = "123456789 977125798 1197932710 "
```

```console
$ CL_HOMEPATH=$(mktemp --directory)
```

```console
$ ${IOQ3} +set cl_allowDownload 1 +set fs_homepath "${CL_HOMEPATH}" +set developer 1 +connect localhost:27960
Need paks: @baseq3/example.pk3@baseq3/example.pk3
URL: http://localhost:3000/baseq3/example.pk3
***** CL_cURL_BeginDownload *****
Localname: baseq3/example.pk3
RemoteURL: http://localhost:3000/baseq3/example.pk3
****************************
writing to: /home/robo9k/.q3a/baseq3/example.pk3.tmp

----- FS_Startup -----
We are looking in the current search path:
/home/robo9k/.q3a/baseq3/example.pk3 (666 files)
    on the pure list

FS search reorder is required
----- FS_Startup -----
We are looking in the current search path:
/home/robo9k/.q3a/baseq3
/home/robo9k/.q3a/baseq3/example.pk3 (666 files)
```

```console
$ curl --location --include --silent http://localhost:3000/baseq3/example.pk3
HTTP/1.1 307 Temporary Redirect
location: https://g-1024.test.mod.io/v1/games/1024/mods/10519/files/14391/download
expires: Wed, 23 Feb 2028 14:49:08 GMT

HTTP/2 302
location: https://binary.test.modcdn.io/mods/…/10519/mymodfile.pk3.zip?verify=…

HTTP/2 200
content-type: application/zip
content-length: 9001
cache-control: public, max-age=2678400
etag: "79054025255fb1a26e4bc422aef54eb4"
last-modified: Tue, 03 Jun 2025 13:03:26 GMT
age: 229597
expires: Thu, 26 Mar 2026 14:49:10 GMT
accept-ranges: bytes
```

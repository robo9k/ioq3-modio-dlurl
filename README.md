```console
$ cargo run
```

```console
$ pk3sum ~/.q3a/baseq3/example.pk3
123456789
```

```console
$ ioq3ded +sets sv_dlURL http://localhost:3000 +map example
]sv_referencedPak
    sv_referencedPakNames = "baseq3/example baseq3/pak8 baseq3/pak4"
    sv_referencedPaks = "123456789 977125798 1197932710 "
```

```console
$ ioq3 +set cl_allowDownload 1 +set developer 1 +connect localhost:27960
Need paks: @baseq3/example.pk3@baseq3/example.pk3
URL: http://localhost:3000/baseq3/example.pk3
***** CL_cURL_BeginDownload *****
Localname: baseq3/example.pk3
RemoteURL: http://localhost:3000/baseq3/example.pk3
****************************
writing to: /home/robo9k/.q3a/baseq3/example.pk3.tmp
```

```console
$ curl -v http://localhost:3000/baseq3/example.pk3
< HTTP/1.1 307 Temporary Redirect
< location: https://api.test.mod.io/v1/games/1024/mods/10519/files/14391/download
< expires: Thu, 17 Apr 2025 16:08:29 GMT
```

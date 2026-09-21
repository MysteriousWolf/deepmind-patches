# Demos

Audio previews, one folder tree mirroring `presets/`. A demo's file name is the
patch stem, plus ` (variant)` for anything but the default take:

```
demos/Bass/Acid Growl - nyx.mp3
demos/Bass/Acid Growl - nyx (mod wheel).mp3
```

Every file here is declared by a `[[demo]]` entry in the patch's `.toml`. CI
refuses a file nothing declares.

| | |
| --- | --- |
| Format | MP3, 128 kbit/s constant, 44.1 kHz, stereo |
| Length | 20 seconds at most |
| Size | 350 KB at most |
| Count | 4 per patch at most |
| Content | The patch alone. No drums, no bed, no mastering. |

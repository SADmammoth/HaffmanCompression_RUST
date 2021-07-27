# Huffman compression

Huffman compression GUI-app [(repo)](https://github.com/SADmammoth/HaffmanCompression/tree/master) rewritten in Rust as CLI-app.

> Application for text compression with Huffman algorithm. Alphabet is generated from the message itself. Compressed message is self-contained and consists of encoded alphabet and payload, and holds all necessary data to decode the message.

## Features

- Compress message and embed alphabet
- Decode alphabet from compressed data and restore original message

## Performance comparision to [previous version](https://github.com/SADmammoth/HaffmanCompression/tree/master)

> - Results of all presented tests are approximate and can't be used for pecisious analysis.
> - Execution time on different machines may differ. Both programs were run on local environment of the same machine.
> - Presented execution time is calculated by embeded features of a language and do not include time for visualization, logging, data output and so on.
> - Rust program has updated version of algorithm that makes it more consistent and effective.
> - Both programs were written without any optimizations or performance tricks.
> - Difference in text parameters depend on the way each language presents text, text files for both programs are identical

### Test 1: W. Shakespeare - Romeo and Juliet

Rust program output:

```
Original size (chars): 142597,
Original size (b): 944182,
Alphabet length (pairs): 65,
Compressed size (b): 689549,
Compressed size (with alphabet, b): 691197,
Compression percent: 26.968634,
Compression percent (with alphabet): 26.794094,
Compression time (ms): 8
Decompression time (ms): 8
```

JS program output:

```
Input chars count: 142597
Compression time: 148.504150390625 ms
Alphabet length: 65
Length in unicode (binary): 944182
Length of compressed in binary: 689549
Length of compressed with alphabet in binary: 691154
Compressed by percent: 26.80
Decompression time: 37.554931640625 ms
```

### Test 2: L. N. Tolstoy - Voyna i mir

Rust program output:

```
Original size (chars): 5570545,
Original size (b): 42571241,
Alphabet length (pairs): 168,
Compressed size (b): 15447710,
Compressed size (with alphabet, b): 15453950,
Compression percent: 63.713276,
Compression percent (with alphabet): 63.69862,
Compression time (ms): 162
Decompression time (ms): 844
```

JS program output:

```
Input chars count: 3326616
Compression time: 12483.6728515625 ms
Alphabet length: 169
Length in unicode (binary): 31296020
Length of compressed in binary: 15992768
Length of compressed with alphabet in binary: 15999067
Compressed by percent: 48.88
Decompression time: 480.153076171875 ms
```

### Test 3: S. King - Under the Dome

Rust program output:

```
Original size (chars): 1835006,
Original size (b): 12447296,
Alphabet length (pairs): 94,
Compressed size (b): 8309033,
Compressed size (with alphabet, b): 8312346,
Compression percent: 33.24628,
Compression percent (with alphabet): 33.219666,
Compression time (ms): 90
Decompression time (ms): 304
```

JS program output:

```
Input chars count: 1829153
Compression time: 1823.005126953125 ms
Alphabet length: 94
Length in unicode (binary): 12418016
Length of compressed in binary: 8454926
Length of compressed with alphabet in binary: 8458261
Compressed by percent: 31.89
Decompression time: 258.366943359375 ms
```

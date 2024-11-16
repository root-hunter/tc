# Tests History

## Version 0.1.0

### WITH ROW DIVISION:
```bash

---- tests::test_1 stdout ----
Original length: 78075
TOKENS: 12796
COMPRESS WITH U16: 12796 < 65535
Compress length: 28032

---- tests::test_4 stdout ----
Original length: 535
TOKENS: 111
COMPRESS WITH U8: 111 < 255
Compress length: 1701
DECOMPRESS WITH U8
TOKENS: 111

---- tests::test_3 stdout ----
Original length: 4343
TOKENS: 717
COMPRESS WITH U16: 717 < 65535
Compress length: 10508
DECOMPRESS WITH U16
TOKENS: 717

---- tests::test_2 stdout ----
Original length: 78075
TOKENS: 12796
COMPRESS WITH U16: 12796 < 65535
Compress length: 28032
DECOMPRESS WITH U16
TOKENS: 12796

---- tests::test_6 stdout ----
Original length: 37350
TOKENS: 4528
COMPRESS WITH U16: 4528 < 65535
Compress length: 30872
DECOMPRESS WITH U16
TOKENS: 4528

---- tests::test_5 stdout ----
Original length: 73437
TOKENS: 10579
COMPRESS WITH U16: 10579 < 65535
Compress length: 130642
DECOMPRESS WITH U16
TOKENS: 10579
```

### WIHOUT ROW DIVISION:
```bash

---- tests::test_4 stdout ----
Original length: 535
TOKENS: 105
COMPRESS WITH U8: 105 < 255
Compress length: 1649
DECOMPRESS WITH U8
TOKENS: 105

---- tests::test_1 stdout ----
Original length: 78075
TOKENS: 12681
COMPRESS WITH U16: 12681 < 65535
Compress length: 27878

---- tests::test_3 stdout ----
Original length: 4343
TOKENS: 683
COMPRESS WITH U16: 683 < 65535
Compress length: 10391
DECOMPRESS WITH U16
TOKENS: 683

---- tests::test_6 stdout ----
Original length: 37350
TOKENS: 3434
COMPRESS WITH U16: 3434 < 65535
Compress length: 28101
DECOMPRESS WITH U16
TOKENS: 3434

---- tests::test_2 stdout ----
Original length: 78075
TOKENS: 12681
COMPRESS WITH U16: 12681 < 65535
Compress length: 27878
DECOMPRESS WITH U16
TOKENS: 12681

---- tests::test_5 stdout ----
Original length: 73437
TOKENS: 10067
COMPRESS WITH U16: 10067 < 65535
Compress length: 126265
DECOMPRESS WITH U16
TOKENS: 10067
```

# Formulas

| name                      | value                   |
| ------------------------- | ----------------------- |
| freq to midi              | 12 * log2(f/440) + 69   |
| midi to freq              | f = 440 * 2^((m−69)/12) |
| freq to cents from A4     | 1200*log2(f/440)        |
| freq to semitones from A4 | 12*log2(f/440)          |

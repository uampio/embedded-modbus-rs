# uamp-modbus-rtu-embbeded-rs

Modbus map - Internally each function start in 0

| Function Code | Description                   | Address Range |
|---------------|-------------------------------|---------------|
| 01 (0x01)     | Read Coils                    | 00001-09999   |
| 02 (0x02)     | Read Discrete Inputs          | 10001-19999   |
| 03 (0x03)     | Read Holding Registers        | 40001-49999   |
| 04 (0x04)     | Read Input Registers          | 30001-39999   |
| 05 (0x05)     | Write Single Coil             | 00001-09999   |
| 06 (0x06)     | Write Single Holding Register | 40001-49999   |
| 15 (0x0F)     | Write Multiple Coils          | 00001-09999   |
| 16 (0x10)     | Write Multiple Holding Registers | 40001-49999   |
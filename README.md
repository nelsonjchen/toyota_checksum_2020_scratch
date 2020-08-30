# Toyota Checksum 2020 Scratch worksheet

This was a worksheet program to work out a checksum scheme for the 2021 Rav4 Prime. It's a series of unit tests and there are some implementations of the legacy weak Toyota Checksum is and incorrect speculation on the new "checksum".

*I failed.*

https://github.com/commaai/openpilot/issues/2103


It doesn't look like a checksum either. The same values should result in the same "checksum". But I see `a20000007f66e2e6` and `a2000000a7ea6089` for `STEERING_LKA`. `a2000000` should result in the same "sum". It doesn't.

* I do not have the skills to figure this out.
* Ways to get the firmware of these vehicles aren't mature. One could reverse engineer the firmware but these vehicles are too new to have public firmware distributed.
* If it's not a sum. Can it even be emulated or MITM'd? 

Whatever this is, it probably means future TSS2 Toyotas may no longer work for openpilot for the time being.

To whoever tries to support new Toyotas, good luck! Maybe "Sell 2021+ Toyota > Buy Hyundai"?
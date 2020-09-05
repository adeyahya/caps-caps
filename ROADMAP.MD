# Roadmap

Motivation
----------

This project was originally started as a complution for me to make a software
that remaping capslock into esc because there's no existing software eq powertoys/karabiner that can run natively on windows 10 arm64 machine
event for the `powertoys` that backed by microsoft who's released Surface Pro X hasn't prioritized it yet
[https://github.com/microsoft/PowerToys/issues/490](https://github.com/microsoft/PowerToys/issues/490)

It started when I was on OS X, there is [Karabiner](https://karabiner-elements.pqrs.org/),
it was quite easy to transform the most useless key ever (for me atleast) in the most useful one
**CAPSLOCK become ESC when it pressed alone, become CTRL/CMD when it pressed alongs with other keys**

![](https://karabiner-elements.pqrs.org/docs/manual/configuration/configure-complex-modifications/images/karabiner-elements-complex-modifications-7@2x.png)

Short Term Goal
----
- [ ] basic/default functionality **CAPSLOCK to ESC if pressed alone, to CTRL/CMD if pressed alongs with other keys**
- [ ] run natively on Mac OS, Linux, Windows (x86 / Arm)
- [ ] easy to toggle interception, eq: using tray on Windows / Mac OS
- [ ] Easy to install & use on all platforms

Long Term Goal
--------------
- [ ] assign CAPSLOCK combination to Macro
- [ ] assign CAPSLOCK combination to run any script / automation (eq: **CAPSLOCK + c to run git checkout -b && git add . && git commit**)
- [ ] `.capscapsrc` config file
- [ ] `you propose`

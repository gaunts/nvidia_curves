# Nvidia-curves

## Disclaimer

This is a work in progress. I am developing it on my free time. As the features implemented are sufficient for me right now, it might stay in this state for some time.
Use at your own risks.

## Description

This project was started to control nvidia fan speeds on Arch-linux. As it turns out, it is also compatible with windows.
I didn't find any solution combining every feature I needed so I started my own.
The combined list of features I am looking for which I couldn't find in any other project :
- Starting as a service in the background.
- CLI app interacting with the running service to switch between profiles.
- GUI helper to edit curves compatible with wayland.
- Compatible with NVidia cards.

This repository includes updated versions of [nvml-wrapper](https://crates.io/crates/nvml-wrapper) and [nvml-wrapper-sys](https://crates.io/crates/nvml-wrapper-sys). I will add them as dependencies when they get updated.

## TODO

- Read curves from a file, with profile management
- Add a client to switch between profiles from the command line
- Add a gui tool to edit curves visually

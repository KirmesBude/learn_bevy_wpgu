# learn_bevy_wgpu

Following https://sotrh.github.io/learn-wgpu/ with bevy

## Dependencies and the window

bevy should take care of including everything we need, so nothing to do here.

##  The Surface

bevy instaniates wgpu Instance, Adapter, RenderDevice, RenderQueue and the Surface.
It also already takes care of repsonding to winit events (e.g. resize).

So to realize what is implemented in render(), we need to look into bevy RenderPasses.

## License

learn_bevy_wgpu is free, open source and permissively licensed!
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](https://github.com/KirmesBude/bevy_titan/blob/main/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/KirmesBude/bevy_titan/blob/main/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!

Most of the code was taken from https://sotrh.github.io/learn-wgpu/, so other licensing might apply.

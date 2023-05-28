# redscript C API
Basic API for decompilation with redscript

## usage
The [decompilation example](examples/decompile.c) can be built and run on Windows:
```powershell
# build the project
cargo build --release
# compile your project with src include path for redscript.h
cl examples/decompile.c target/release/redscript_capi.dll.lib -Isrc
# copy the shared lib
copy target\release\redscript_capi.dll .
# running the demo should output the decompiled code
decompile.exe
```
*the above was tested in `x64 Native Tools Command Prompt for VS 2022`*

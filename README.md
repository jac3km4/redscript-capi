# redscript C API
Basic API for decompilation with redscript

## usage
The [decompilation example](examples/decompile.c) can be built and run on Windows:
```powershell
# build the project
cargo build --release
# compile your project with include path containing redscript.h
cl examples/decompile.c target/release/redscript_capi.dll.lib -Isrc
# copy the previously compiled shared lib
copy target\release\redscript_capi.dll .
# running the demo should output the decompiled code
decompile.exe
# the example has a hardcoded path that you'll probably need to update for it to work on your machine
```
*the above was tested in `x64 Native Tools Command Prompt for VS 2022`*

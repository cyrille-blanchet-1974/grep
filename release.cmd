@cargo build --release
@if exist ..\bin\nul copy target\release\*.exe ..\bin
@pause
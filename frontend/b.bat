npm run build
del /S /Q ..\src\resources\assets\*
xcopy /S dist\* ..\src\resources\assets\.
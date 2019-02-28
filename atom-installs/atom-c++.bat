@echo off

:: USER VARIABLES

:: The path to the intended atom home directory.
:: Use "%~dp0" followed by your folder name for a folder in the CWD. Ex: "%~dp0atominstallone"
set atomdir=%~dp0c++


:: MAIN SCRIPT

:: Create the 'atomdir' folder if it doesn't already exist
if not exist "%atomdir%" (
    mkdir "%atomdir%"
)

:: Set the ATOM_HOME environement variable to 'atomdir' for the duration of the batch script
set ATOM_HOME=%atomdir%

:: Launch Atom
atom.cmd

# Razer Keyboard Mode Changer

This will set Razer keyboards to some kind of factory testing mode that will make the macro keys behave as macro keys. This should allow them to be used without having to have the Razer software installed.

## Install

Download the latest release from the github releases. You can then copy it somewhere on your system and install it by running the following in an Administrator CMD:

```bat
razer_keyboard_mode_changer.exe --install
```

This will add a scheduled task that will reset the device mode of the keyboard on every wake from sleep.

## How to use

You can just run the command without any parameters to set the special device mode.
You can then use something like Autohotkey to program macros. Or you can just let any given program bind to the F13-F17 keys.

## Uninstall

```bat
razer_keyboard_mode_changer.exe --uninstall
```

## Contributing

Pull requests are welcome. Please follow [Conventional Commits](https://www.conventionalcommits.org) to make the changelog work.

## License

[MIT](https://choosealicense.com/licenses/mit/)

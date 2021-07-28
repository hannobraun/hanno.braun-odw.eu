# made-by.braun-odw.eu

## Server

To update server configuration after changing it:

``` bash
cd server
nixos-rebuild switch --flake .
```

To upgrade the packages installed on the server:

``` bash
cd server
nixos-rebuild switch --flake . --upgrade
```

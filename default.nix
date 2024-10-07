
with import <nixpkgs> { };

mkShell {

  nativeBuildInputs = [
     rustup
     SDL2
     iconv
   ];
   shellHook = ''
     fish
   '';
}


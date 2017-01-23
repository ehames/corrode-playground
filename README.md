# corrode-playground
A web interface to test corrode (https://github.com/jameysharp/corrode).

## Setup
1. Set up the build environment. Checkout [Rust dev env](https://github.com/ehames/rust-devenv).

1. Build binaries for corrode and corrode-playground.
   ```bash
   # build corrode
   cd $PROJDIR
   git clone https://github.com/jameysharp/corrode.git
   cd corrode
   stack build

   # build corrode-playground
   cd $PROJDIR
   git clone https://github.com/ehames/corrode-playground
   cd corrode-playground
   ~/.cargo/bin/cargo build

   # copy binaries
   cp $PROJDIR/corrode/.stack-work/install/x86_64-linux/lts-6.6/7.10.3/bin/corrode .
   cp $PROJDIR/corrode-playground/target/debug/corrode_playground .

   # build docker image
   docker build -t corrode-playground:latest .
   ```

1. Build and run the web service
    ```bash
    docker run -it --rm -p 6767:6767 -e RUST_LOG=debug --name corrode-playground corrode-playground:latest
    ```

1. Try sending some C code to the web service

   ```bash
   curl -X POST --data 'ccode=int main(){ return 0; }' http://localhost:6767/corrode
   ```

1. Open your browser and checkout the webapp at http://localhost:6767/

## Authors
  [Edgardo Hames](https://github.com/ehames)
  [Francisco Ferreira](https://github.com/fferreira)

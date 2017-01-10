# corrode-playground
A web interface to test corrode (https://github.com/jameysharp/corrode)

## Setup
1. Build a Docker image with corrode

    ```bash
    docker build -t corrode:latest .
    ```

1. Build and run the web service

    ```bash
    cargo run
    ```

1. Try sending some C code to the web service

   ```bash
   curl -X POST --data 'ccode=int main(){ return 0; }' http://localhost:6767/corrode
   ```

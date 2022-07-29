# Book a slot
Bookaslot is a personal project to learn single web application technologies with Svelte as a frontend technology and Rust language as a backend with the web server Rocket and the "ORM" Diesel.

With Bookaslot you can book slots, for example, you can book your tennis court, squash court, a doctor appointment, etc...

## Config
Configure the .env file to fit your requirements. The current .env file is an example and it is used for local development. Make sure your production .env file is not readable by others than you.

## System design
The entry point is a nginx reverse-proxy. The reverse-proxy follows the traffic to another nginx container where the frontend application with svelte technology runs. It also forward the /api requests to the backend.

In total there are 4 containers running, the database which is accessed by the backend (or api) container, the web container which contains the frontend application and finally the nginx-reverse proxy.

* nginx-proxy: is the reverse proxy container.
* web: is the containerized frontend application developed with svelte.
* backend: is the Rest-API developed with rust running into a container. It uses the Rocket crate as web-server and the Diesel crate to access the database.
* postgres: this is the database container.

## Usage

### Development mode

<details><summary><b>Show instructions</b></summary>

1. Run docker-compose:

    ```sh
    $ docker-compose  up --build
    ```
2. Open your browser and visit the different options for testing
    * The application running at 0.0.0.0 (http://0.0.0.0)
    * The frontend running at 0.0.0.0:5000 (http://0.0.0.0:5000)
    * The backend running at 0.0.0.0:8000 (http://0.0.0.0:5000)


</details>

### Production mode

<details><summary><b>Show instructions</b></summary>

1. Make sure you have the right image version in your .env file. The variable BOOKASLOT_VERSION
must contain the value of the desired version that you want to run:

    ```sh
    $ cat .env | grep BOOKASLOT_VERSION
    ```

2. Run the database from the root directory:

    ```sh
    $ make start-prod
    ```
</details>

### Backend

<details><summary><b>Show instructions</b></summary>

1. Run the database from the root directory:

    ```sh
    $ make psql
    ```

2. Enter in the backend directory:

    ```sh
    $ cd backend
    ```

3. Run cargo:

    ```sh
    $ cargo run
    ```
</details>

### Frotend

<details><summary><b>Show instructions</b></summary>

The frontend is the hardest part to test beacause it depends in the backen which also depends on the database. For that reason the you can do it, is to follow the Development mode instructions, which sets up everything for you, including dealing with CORS problems.

Otherwise you can start the frontend following the README.md file at the web directory.
</details>
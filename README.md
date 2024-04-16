# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create SubscriberRequest model struct.`
    -   [ ] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Notification repository.`
    -   [ ] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Commit: `Implement receive_notification function in Notification service.`
    -   [ ] Commit: `Implement receive function in Notification controller.`
    -   [ ] Commit: `Implement list_messages function in Notification service.`
    -   [ ] Commit: `Implement list function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1
1. Penggunaan RwLock<> dibandingkan dengan Mutex<> dalam kasus ini diperlukan karena RwLock<> memungkinkan banyak thread untuk membaca variabel secara bersamaan, sementara Mutex<> hanya mengizinkan satu thread untuk menggunakan variabel pada suatu waktu. Dalam konteks ini, Vec of Notifications akan sering dibaca oleh banyak thread tanpa penulisan. Jika menggunakan Mutex<>, hal tersebut akan menghambat kinerja karena hanya satu thread yang dapat mengakses variabel pada satu waktu.

2. Penggunaan library external lazy_static digunakan untuk membuat variabel menjadi singleton, yang berarti hanya ada satu instance dari variabel tersebut dalam program. Selain itu, Rust membuat variabel static menjadi immutable secara default, berbeda dengan Java yang memungkinkan perubahan nilai variabel melalui fungsi statis. Hal ini dilakukan oleh Rust untuk menjamin keamanan thread saat digunakan dalam lingkungan multithreading, karena data yang immutable lebih mudah dikelola dan aman dari race condition.

#### Reflection Subscriber-2
1. Saya sudah mengeksplorasi beberapa bagian di luar langkah-langkah yang dijelaskan dalam tutorial, termasuk src/lib.rs. Dari pemeriksaan tersebut, saya memahami bahwa lib.rs berfungsi sebagai modul utama yang menyediakan informasi penting yang digunakan oleh bagian-bagian lain dari aplikasi. Ini mencakup penanganan error, definisi root URL, dan pembuatan singleton untuk konfigurasi aplikasi.

2. Ya, dengan pola Observer yang diterapkan, menambahkan lebih banyak subscriber menjadi lebih mudah karena struktur program sudah dirancang dengan prinsip open-close, yang memungkinkan untuk menambah fungsionalitas tanpa mengubah kode yang sudah ada. Ketika melakukan instansiasi lebih dari satu Main App, masih memungkinkan karena mendaftarkan subscriber ke aplikasi yang berbeda dapat dilakukan dengan mengakses API yang sesuai.

3. Saya telah mencoba membuat beberapa tes sendiri dan meningkatkan dokumentasi pada koleksi Postman saya. Menurut saya, fitur ini sangat berguna karena membantu saya memverifikasi kebenaran program dan memastikan bahwa respons yang dihasilkan sesuai dengan harapan. Koleksi Postman juga membantu dalam memastikan bahwa aplikasi berjalan seperti yang diharapkan dengan menggunakan data nyata dari aplikasi itu sendiri.